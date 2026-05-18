mod server;

use rmcp::{ServiceExt, transport::stdio};
use server::QueueMcpServer;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn queue_api_exe() -> PathBuf {
    // queue-mcp.exe: MCPQueue/queue-mcp/target/release/
    // queue-api.exe: MCPQueue/queue-api/target/release/
    env::current_exe()
        .ok()
        .and_then(|p| p.parent()?.parent()?.parent()?.parent().map(|r| {
            r.join("queue-api")
             .join("target")
             .join("release")
             .join("queue-api.exe")
        }))
        .unwrap_or_else(|| PathBuf::from("queue-api.exe"))
}

async fn ensure_api_running() {
    let client = reqwest::Client::new();
    let health = format!("http://localhost:3000/queue/size");

    if client.get(&health).timeout(Duration::from_millis(300)).send().await.is_ok() {
        return; // 이미 실행 중
    }

    let exe = queue_api_exe();
    if let Err(_) = std::process::Command::new(&exe).spawn() {
        return;
    }

    // 최대 3초 대기
    for _ in 0..15 {
        tokio::time::sleep(Duration::from_millis(200)).await;
        if client.get(&health).timeout(Duration::from_millis(300)).send().await.is_ok() {
            break;
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ensure_api_running().await;
    let service = QueueMcpServer::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
