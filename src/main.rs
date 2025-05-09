use axum::{Router, routing::{get, post}, response::IntoResponse, extract::{Json, State}};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::net::SocketAddr;

mod worker;

#[tokio::main]
async fn main() {
    // Start the server
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let state = Arc::new(Mutex::new(AppState::default()));

    let app = Router::new()
        .route("/", get(hello))
        .route("/register", post(register_worker))
        .route("/submit", post(submit_job))
        .with_state(state.clone());

    println!("🚀 Server running on http://{}", addr);

    if let Err(e) = axum::serve(TcpListener::bind(addr).await.unwrap(), app).await {
        eprintln!("Server error: {:?}", e);
    }
}

async fn hello() -> &'static str {
    "GPU Pool Server Online!"
}

#[derive(Debug, Default)]
struct AppState {
    workers: HashMap<String, String>,
    jobs: Vec<Job>,
}

#[derive(Serialize, Deserialize)]
struct RegisterWorker {
    address: String,
}

#[derive(Serialize, Deserialize)]
struct RegisterResponse {
    worker_id: String,
}

async fn register_worker(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<RegisterWorker>,
) -> impl IntoResponse {
    let worker_id = Uuid::new_v4().to_string();
    let mut state = state.lock().unwrap();
    state.workers.insert(worker_id.clone(), payload.address);
    Json(RegisterResponse { worker_id })
}

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    id: String,
    data: String,
}

async fn submit_job(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<Job>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    let (worker_id, addr) = match state.workers.iter().next() {
        Some((id, addr)) => (id.clone(), addr.clone()),
        None => return Json("No workers available!".to_string()),
    };

    let job_id = Uuid::new_v4().to_string();
    let job = Job {
        id: job_id.clone(),
        data: payload.data,
    };
    state.jobs.push(job);

    println!("📨 Sent job {} to worker {} at {}", job_id, worker_id, addr);

    Json(format!("Job {} sent to worker {}!", job_id, worker_id))
}
