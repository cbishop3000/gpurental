use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use uuid::Uuid;


#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::default()));

    let app = Router::new()
        .route("/", get(hello))
        .route("/register", post(register_worker))
        .route("/submit", post(submit_job))
        .with_state(state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server running on http://{addr}");

    axum::serve(
    tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn hello() -> &'static str {
    "GPU Pool Server Online!"
}

#[derive(Debug, Default)]
struct AppState {
    workers: HashMap<String, String>,
    jobs: Vec<Job>,
}

#[derive(Debug, Deserialize)]
struct RegisterWorker {
    address: String,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
struct SubmitJob {
    job_data: String,
}

#[derive(Debug)]
struct Job {
    id: String,
    data: String,
}

async fn submit_job(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<SubmitJob>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    // First, extract a worker ID and addr into owned variables (clone them)
    let (worker_id, addr) = match state.workers.iter().next() {
        Some((id, addr)) => (id.clone(), addr.clone()),
        None => return Json("No workers available!".to_string()),
    };

    // Now that immutable borrow is done, you can mutate
    let job_id = Uuid::new_v4().to_string();
    let job = Job {
        id: job_id.clone(),
        data: payload.job_data,
    };
    state.jobs.push(job);

    println!("📨 Sent job {job_id} to worker {worker_id} at {addr}");

    Json(format!("Job {} sent to worker {}!", job_id, worker_id))
}

