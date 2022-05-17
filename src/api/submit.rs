use serde::{Deserialize, Serialize};

use crate::header::get_header_map;

use super::{get_exam_id, paper::Paper};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Remark {
    pub is_vidator: bool,
    pub opinion: String,
    pub questiont_id: String,
    pub score: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReq {
    pub examination_id: String,
    pub student_id: String,
    pub remark_list: Vec<Remark>,
}

#[derive(Deserialize, Debug)]
pub struct SubmitResp {
    code: i32,
    msg: String,
    success: bool,
}

fn req_from_paper(student_id: &str, paper: &Paper) -> SubmitReq {
    let mut remark_list = Vec::new();
    let exam_id = get_exam_id();
    for part in &paper.paper_parts {
        for node in &part.paper_trees {
            remark_list.push(Remark {
                is_vidator: false,
                opinion: "".to_string(),
                questiont_id: node.question_id.clone(),
                score: node.score.to_string(),
            });
        }
    }
    SubmitReq {
        examination_id: exam_id,
        student_id: student_id.to_string(),
        remark_list: remark_list,
    }
}

pub async fn post_submit(student_id: &str, paper: &Paper) -> anyhow::Result<()> {
    let req = req_from_paper(student_id, paper);
    let client = reqwest::Client::new();
    let resp = client
        .post("https://apiucloud.bupt.edu.cn/ykt-site/answersheet/submit-remark")
        .json(&req)
        .headers(get_header_map()?)
        .send()
        .await?
        .text()
        .await?;
    println!("{} \n\n {}", serde_json::to_string(&req)?, resp);
    let resp: SubmitResp = serde_json::from_str(&resp)?;
    if resp.success {
        Ok(())
    } else {
        Err(anyhow::anyhow!("{}", resp.msg))
    }
}
