mod queue;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use queue::Queue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};

type SharedQueue = Arc<Mutex<Queue>>;

#[derive(Deserialize)]
struct EnqueueRequest {
    data: Value,
}

#[derive(Serialize)]
struct SizeResponse {
    size: usize,
}

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

// POST /queue  — enqueue
async fn enqueue(
    State(q): State<SharedQueue>,
    Json(payload): Json<EnqueueRequest>,
) -> (StatusCode, Json<Value>) {
    let item = q.lock().unwrap().enqueue(payload.data);
    (StatusCode::CREATED, Json(serde_json::to_value(item).unwrap()))
}

// GET /queue  — list all items
async fn list(State(q): State<SharedQueue>) -> Json<Value> {
    let items = q.lock().unwrap().list();
    Json(serde_json::to_value(items).unwrap())
}

// DELETE /queue  — clear
async fn clear(State(q): State<SharedQueue>) -> Json<MessageResponse> {
    q.lock().unwrap().clear();
    Json(MessageResponse { message: "Queue cleared".to_string() })
}

// GET /queue/front  — peek
async fn peek(State(q): State<SharedQueue>) -> (StatusCode, Json<Value>) {
    match q.lock().unwrap().peek() {
        Some(item) => (StatusCode::OK, Json(serde_json::to_value(item).unwrap())),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Queue is empty"}))),
    }
}

// DELETE /queue/front  — dequeue
async fn dequeue(State(q): State<SharedQueue>) -> (StatusCode, Json<Value>) {
    match q.lock().unwrap().dequeue() {
        Some(item) => (StatusCode::OK, Json(serde_json::to_value(item).unwrap())),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Queue is empty"}))),
    }
}

// GET /queue/size
async fn size(State(q): State<SharedQueue>) -> Json<SizeResponse> {
    Json(SizeResponse { size: q.lock().unwrap().size() })
}

#[tokio::main]
async fn main() {
    let shared_queue: SharedQueue = Arc::new(Mutex::new(Queue::new()));

    let app = Router::new()
        .route("/queue", post(enqueue).get(list).delete(clear))
        .route("/queue/front", get(peek).delete(dequeue))
        .route("/queue/size", get(size))
        .with_state(shared_queue);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Queue API running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
