use crate::api::paper::Paper;
use crate::api::set_question_id;
use crate::api::{list::Student, set_exam_id};
use crate::db;
use crate::{api, header};
use anyhow::{Context, Result};
use axum::extract::Form;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::Router;
use futures::executor::block_on;
use once_cell::sync::Lazy;
use sea_orm::DbConn;
use serde::Deserialize;
use std::{collections::VecDeque, sync::Mutex};

static STUDENT_LIST: Lazy<Mutex<VecDeque<Student>>> = Lazy::new(|| Mutex::new(VecDeque::new()));
static CURRENT_STUDENT: Lazy<Mutex<Option<(Student, Paper)>>> = Lazy::new(|| Mutex::new(None));
static DB_CONN: Lazy<DbConn> = Lazy::new(|| block_on(crate::db::connect()).unwrap());

fn get_db_conn() -> &'static DbConn {
    &*DB_CONN
}

pub async fn init(
    auth: String,
    blade: String,
    identity: String,
    exam_id: String,
    question_id: String,
) -> Result<()> {
    header::set_authorization(auth);
    header::set_blade_auth(blade);
    header::set_identity(identity);
    set_exam_id(exam_id);
    set_question_id(question_id);

    let students = api::list::get_students().await?;
    {
        let students = {
            let mut result = Vec::new();
            for i in students {
                if db::get_student_by_id(get_db_conn(), &i.student_id).await?.is_none() {
                    result.push(i);
                }
            }
            result
        };
        let mut guard = STUDENT_LIST.lock().unwrap();
        guard.extend(students.into_iter());
    }

    let app = Router::new().route(
        "/",
        get(|| async { handler().await }).post(|req| async { accept_form(req).await }),
    );

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn fetch_current_student() -> Result<Option<(Student, Paper)>> {
    let student = STUDENT_LIST.lock().unwrap().pop_front();
    if let Some(student) = student {
        let paper = api::paper::get_paper(&student.student_id).await?;
        let mut guard = CURRENT_STUDENT.lock().unwrap();
        let result = Some((student, paper));
        *guard = result.clone();
        Ok(result)
    } else {
        Ok(None)
    }
}

async fn get_current_student() -> Result<Option<(Student, Paper)>> {
    let student = CURRENT_STUDENT.lock().unwrap().clone();
    if let Some(student) = student {
        Ok(Some(student.clone()))
    } else {
        fetch_current_student().await
    }
}

async fn handler() -> Html<String> {
    Html(if let Ok(resp) = index_inner().await {
        resp
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.to_string()
    })
}

async fn index_inner() -> Result<String> {
    if let Some((student, paper)) = get_current_student().await? {
        let name = student.student_name;
        let question_id = api::get_question_id();
        let question = paper
            .get_question_info(&question_id)
            .context("get_question_info")?;
        let db_student = db::get_student_by_id(get_db_conn(), &student.student_id).await?;
        let (score, source) = if let Some(s) = db_student {
            (s.score, "db")
        } else {
            (question.score, "api")
        };
        Ok(format!(
            r#"
<!DOCTYPE html>
<style>
img {{
    max-width: 100%;
    max-height: 1000px;
    height: auto;
}}
</style>
<h1>{}</h1>
{}
<h3>答案</h3>
<form action="/" method="POST">
    <input name="points" id="points" value="{}"> / {} ({})
    <button>提交</button>
</form>

{}
                "#,
            name, question.stem, score, question.points, source, question.answer,
        ))
    } else {
        Ok("<h1>No more students</h1>".to_string())
    }
}

#[derive(Deserialize)]
struct Remark {
    points: String,
}

async fn accept_form(form: Form<Remark>) -> Html<String> {
    Html(if let Ok(resp) = accept_form_inner(form).await {
        resp
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.to_string()
    })
}

async fn accept_form_inner(form: Form<Remark>) -> Result<String> {
    let remark: Remark = form.0;
    let (student, mut paper) = CURRENT_STUDENT
        .lock()
        .unwrap()
        .take()
        .context("failed to take")?;
    let question_id = api::get_question_id();
    db::upsert_student(
        get_db_conn(),
        &student.student_id,
        &student.student_name,
        &remark.points,
    ).await?;
    paper.update_question(&question_id, &remark.points);
    //api::submit::post_submit(&student.student_id, &paper).await?;
    index_inner().await
}
