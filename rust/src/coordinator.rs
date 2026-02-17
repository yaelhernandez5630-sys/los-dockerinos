use axum::{routing::{get, post}, Json, Router};
use std::net::SocketAddr;
use crate::models::{Task, TaskResult};

pub async fn run_coordinator() {
    println!("🚀 Iniciando Coordinador en 10.10.10.1:3000...");

    let app = Router::new()
        .route("/get_task", get(assign_task))
        .route("/submit_result", post(receive_result));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn assign_task() -> Json<Task> {
    // Tarea Dummy
    let task = Task {
        task_id: 101,
        command: "CALCULAR_MANDELBROT_DUMMY".to_string(),
    };
    println!("Coordinador: Asignando tarea {} a un worker.", task.task_id);
    Json(task)
}

async fn receive_result(Json(result): Json<TaskResult>) {
    println!(
        "✅ Coordinador: Resultado recibido de {} para la tarea {}. Estado: {}",
        result.worker_id, result.task_id, result.status
    );
}