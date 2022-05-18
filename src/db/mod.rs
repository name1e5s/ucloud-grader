pub mod entity;

use anyhow::Result;
use sea_orm::{
    ActiveValue::NotSet, ColumnTrait, ConnectionTrait, Database, DbConn, IntoActiveModel,
    QueryFilter, Schema, ActiveModelBehavior, Set,
};
use std::{env, fs::File, path::Path};

const DEFAULT_DB_URL_PREFIX: &str = "sqlite:";
const DEFAULT_DB_FILE: &str = "m.db";

pub fn get_db_file() -> String {
    env::var("M_DB_PATH").unwrap_or(DEFAULT_DB_FILE.to_owned())
}

fn get_db_url() -> String {
    let file_path = env::var("M_DB_PATH").unwrap_or(DEFAULT_DB_FILE.to_owned());
    let mut url = DEFAULT_DB_URL_PREFIX.to_owned();
    url.push_str(&file_path);
    eprintln!("DB_URL = {}", url);
    url
}

pub async fn connect() -> Result<DbConn> {
    let db_file = get_db_file();
    let exists = Path::new(&db_file).exists();
    if !exists {
        let file = File::create(&db_file)?;
        drop(file);
        let conn = Database::connect(get_db_url()).await?;
        setup_schema(&conn).await?;
        Ok(conn)
    } else {
        Ok(Database::connect(get_db_url()).await?)
    }
}

pub async fn setup_schema(db_conn: &DbConn) -> Result<()> {
    let backend = db_conn.get_database_backend();
    let schema = Schema::new(backend);
    let create_student_stmt = schema.create_table_from_entity(entity::Entity);
    db_conn
        .execute(db_conn.get_database_backend().build(&create_student_stmt))
        .await?;
    Ok(())
}

pub async fn get_student_by_id(db_conn: &DbConn, id: &str) -> Result<Option<entity::Model>> {
    use sea_orm::EntityTrait;
    Ok(entity::Entity::find()
        .filter(entity::Column::Sid.eq(id))
        .one(db_conn)
        .await?)
}

pub async fn insert_student(db_conn: &DbConn, mut student: entity::ActiveModel) -> Result<()> {
    use sea_orm::ActiveModelTrait;
    student.id = NotSet;
    student.insert(db_conn).await?;
    Ok(())
}

pub async fn update_student(db_conn: &DbConn, student: entity::ActiveModel) -> Result<()> {
    use sea_orm::ActiveModelTrait;
    student.update(db_conn).await?;
    Ok(())
}

pub async fn upsert_student(db_conn: &DbConn, id: &str, name: &str, score: &str) -> Result<()> {
    if let Some(student) = get_student_by_id(db_conn, id).await? {
        let mut student = student.into_active_model();
        student.name = Set(name.to_owned());
        student.score = Set(score.to_owned());
        update_student(db_conn, student).await?;
    } else {
        let mut student = entity::ActiveModel::new();
        student.sid = Set(id.to_owned());
        student.name = Set(name.to_owned());
        student.score = Set(score.to_owned());
        insert_student(db_conn, student).await?;
    }
    Ok(())
}
