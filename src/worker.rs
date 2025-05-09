use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct RegisterWorker {
    address: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponse {
    worker_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    id: String,
    data: String,
}

#[tokio::main]
async fn main() {
    let server_address = "http://10.0.0.75:3000"; // Update with your Axum server's IP address
    let worker_address = "192.168.56.1"; // The address of this worker

    let client = Client::new();

    // Register the worker
    let register_worker = RegisterWorker {
        address: worker_address.to_string(),
    };

    let register_response: RegisterResponse = client
        .post(format!("{}/register", server_address))
        .json(&register_worker)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let worker_id = register_response.worker_id;
    println!("Registered worker with ID: {}", worker_id);

    // Simulate submitting a job (you can replace this with actual GPU task logic)
    let job = Job {
        id: Uuid::new_v4().to_string(),
        data: "process this job".to_string(),
    };

    let response = client
        .post(format!("{}/submit", server_address))
        .json(&job)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Server response: {}", response);
}
