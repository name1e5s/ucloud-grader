use crate::header::get_header_map;
use serde::{Deserialize, Serialize};

use super::{get_exam_id, list::Score};
use anyhow::Result;

#[derive(Serialize, Debug, Clone)]
pub struct QuestionInfo {
    pub id: String,
    pub question_id: String,
    pub points: String,
    pub score: String,
    pub stem: String,
    pub answer: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub id: String,
    pub stem: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaperNode {
    pub sort: u32,
    pub id: String,
    pub question_id: String,
    pub points: String,
    pub question_combination_d_t_o: Question,
    #[serde(default)]
    pub score: Score,
    pub self_answer: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaperPart {
    pub sequence: u32,
    pub paper_trees: Vec<PaperNode>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Paper {
    pub paper_parts: Vec<PaperPart>,
}

impl Paper {
    pub fn get_question_info(&self, id: &str) -> Option<QuestionInfo> {
        for part in &self.paper_parts {
            for node in &part.paper_trees {
                if node.question_id == id {
                    return Some(QuestionInfo {
                        id: node.id.clone(),
                        question_id: node.question_id.clone(),
                        points: node.points.clone(),
                        score: node.score.to_string(),
                        stem: node.question_combination_d_t_o.stem.clone(),
                        answer: node.self_answer.join("\n"),
                    });
                }
            }
        }
        None
    }

    pub fn update_question(&mut self, question_id: &str, score: &str) {
        for part in &mut self.paper_parts {
            for node in &mut part.paper_trees {
                if node.question_id == question_id {
                    node.score = Score::Str(score.to_string());
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct GetPaperResp {
    code: i32,
    msg: String,
    success: bool,
    data: Paper,
}

pub async fn get_paper(student_id: &str) -> Result<Paper> {
    let url = format!(
        "https://apiucloud.bupt.edu.cn/ykt-site/answersheet/have-read-paper-tea?examinationId={}&studentId={}",
        get_exam_id(),
        student_id,
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .headers(get_header_map()?)
        .send()
        .await?
        .text()
        .await?;
    let resp: GetPaperResp = serde_json::from_str(&resp)?;
    Ok(resp.data)
}
