pub mod header;
pub mod api;
pub mod server;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let auth = "YOUR authorization".to_string();
    let blade = "YOUR blade-auth".to_string();
    let identity = "YOUR identity".to_string();
    let exam_id = "ID in url".to_string();
    let question_id = "ID in resp of have-read-paper-tea".to_string();
    server::init(auth, blade, identity, exam_id, question_id).await?;
    Ok(())
}
