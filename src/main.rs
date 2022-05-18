pub mod api;
pub mod db;
pub mod header;
pub mod server;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let auth = "Basic c3dvcmQ6c3dvcmRfc2VjcmV0".to_string();
    let blade = "".to_string();
    let identity = "JS006:1401097964818010113".to_string();
    let exam_id = "1524325793075015681".to_string();
    let question_id = "1523962582352412673".to_string();
    server::init(auth, blade, identity, exam_id, question_id).await?;
    Ok(())
}
