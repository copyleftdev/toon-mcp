use rmcp::{transport::stdio, ServiceExt};
use crate::tools::ToonTools;

pub async fn run_server() -> anyhow::Result<()> {
    let service = ToonTools::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
