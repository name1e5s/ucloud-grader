use serde::Deserialize;
use anyhow::Result;
use super::get_exam_id;
use crate::header::get_header_map;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Score {
    Str(String),
    Int(i32),
}

impl ToString for Score {
    fn to_string(&self) -> String {
        match self {
            Score::Str(s) => s.to_string(),
            _ => "0".to_string(),
        }
    }
}

impl Default for Score {
    fn default() -> Self {
        Score::Int(0)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    pub grade_id: String,
    pub grade_name: String,
    pub student_id: String,
    pub student_name: String,
    pub submit_exam: bool,
    #[serde(default)]
    pub score: Score,
}

#[derive(Deserialize)]
pub struct GetStudentListResp {
    code: i32,
    msg: String,
    success: bool,
    data: Vec<Student>,
}

pub async fn get_students() -> Result<Vec<Student>> {
    let url = format!(
        "https://apiucloud.bupt.edu.cn/ykt-site/examination/check-list?id={}&status=-1",
        get_exam_id()
    );
    let client = reqwest::Client::new();
    let resp = client.get(&url).headers(get_header_map()?).send().await?.text().await?;
    println!("{}", resp);
    let resp: GetStudentListResp = serde_json::from_str(&resp)?;
    Ok(resp.data)
}