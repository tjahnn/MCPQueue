#![allow(dead_code)]

use rmcp::{ServerHandler, model::{ServerCapabilities, ServerInfo}, schemars, tool};
use serde::Deserialize;
use std::sync::Arc;

const API_BASE: &str = "http://localhost:3000";

#[derive(Debug, Clone)]
pub struct QueueMcpServer {
    client: Arc<reqwest::Client>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EnqueueParams {
    /// 큐에 추가할 데이터
    data: String,
}

#[tool(tool_box)]
impl QueueMcpServer {
    pub fn new() -> Self {
        Self { client: Arc::new(reqwest::Client::new()) }
    }

    #[tool(description = "Add data to the back of the queue")]
    async fn enqueue(&self, #[tool(aggr)] EnqueueParams { data }: EnqueueParams) -> String {
        let body = serde_json::json!({ "data": data });
        self.request("POST", "/queue", Some(body)).await
    }

    #[tool(description = "Remove and return the front item from the queue")]
    async fn dequeue(&self) -> String {
        self.request("DELETE", "/queue/front", None).await
    }

    #[tool(description = "View the front item without removing it")]
    async fn peek(&self) -> String {
        self.request("GET", "/queue/front", None).await
    }

    #[tool(description = "List all items in the queue in FIFO order")]
    async fn list_queue(&self) -> String {
        self.request("GET", "/queue", None).await
    }

    #[tool(description = "Get the number of items currently in the queue")]
    async fn queue_size(&self) -> String {
        self.request("GET", "/queue/size", None).await
    }

    #[tool(description = "Remove all items from the queue")]
    async fn clear_queue(&self) -> String {
        self.request("DELETE", "/queue", None).await
    }
}

impl QueueMcpServer {
    async fn request(&self, method: &str, path: &str, body: Option<serde_json::Value>) -> String {
        let url = format!("{}{}", API_BASE, path);
        let builder = match method {
            "POST"   => self.client.post(&url),
            "DELETE" => self.client.delete(&url),
            _        => self.client.get(&url),
        };
        let builder = match body {
            Some(b) => builder.json(&b),
            None    => builder,
        };
        match builder.send().await {
            Ok(r)  => r.text().await.unwrap_or_else(|e| e.to_string()),
            Err(e) => format!("{{\"error\": \"{e}\"}}"),
        }
    }
}

// impl ServerHandler에도 #[tool(tool_box)]가 있어야 list_tools/call_tool이 자동 주입됨
#[tool(tool_box)]
impl ServerHandler for QueueMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Queue MCP server. Calls queue-api (http://localhost:3000). \
                 Tools: enqueue, dequeue, peek, list_queue, queue_size, clear_queue."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
