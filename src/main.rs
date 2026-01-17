mod error;
mod server;
mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("toon-mcp server starting...");
    server::run_server().await
}
