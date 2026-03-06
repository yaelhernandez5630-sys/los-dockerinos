use axum::{routing::{get, post}, Json, Router, extract::State};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::env;
use crate::models::{Task, TaskResult, MandelbrotParams, ImageRegion};

#[derive(Clone)]
struct CoordinatorState {
    pending_tasks: Arc<Mutex<Vec<Task>>>,
    completed_results: Arc<Mutex<HashMap<u32, TaskResult>>>,
    next_task_id: Arc<Mutex<u32>>,
    total_tasks: Arc<Mutex<u32>>,
    mandelbrot_params: Arc<Mutex<MandelbrotParams>>,
}

pub async fn run_coordinator() {
    println!("🚀 Iniciando Coordinator...");
    
    // Obtener configuración de variables de entorno
    let port = env::var("COORDINATOR_PORT").unwrap_or_else(|_| "3000".to_string());
    let width: u32 = env::var("IMAGE_WIDTH").unwrap_or_else(|_| "800".to_string()).parse().unwrap_or(800);
    let height: u32 = env::var("IMAGE_HEIGHT").unwrap_or_else(|_| "600".to_string()).parse().unwrap_or(600);
    let regions_per_task: u32 = env::var("REGIONS_PER_TASK").unwrap_or_else(|_| "4".to_string()).parse().unwrap_or(4);
    
    // Parámetros de Mandelbrot
    let params = MandelbrotParams {
        width,
        height,
        x_min: -2.5,
        x_max: 1.0,
        y_min: -1.25,
        y_max: 1.25,
        max_iterations: 256,
    };
    
    // Generar tareas iniciales
    let tasks = generate_tasks(&params, regions_per_task);
    let total_tasks = tasks.len() as u32;
    
    let state = CoordinatorState {
        pending_tasks: Arc::new(Mutex::new(tasks)),
        completed_results: Arc::new(Mutex::new(HashMap::new())),
        next_task_id: Arc::new(Mutex::new(0)),
        total_tasks: Arc::new(Mutex::new(total_tasks)),
        mandelbrot_params: Arc::new(Mutex::new(params)),
    };
    
    println!("📊 Coordinator: Generadas {} tareas para imagen {}x{}", total_tasks, width, height);
    println!("🌐 Coordinator escuchando en 0.0.0.0:{}", port);
    
    let app = Router::new()
        .route("/get_task", get(assign_task))
        .route("/submit_result", post(receive_result))
        .route("/status", get(get_status))
        .route("/health", get(health_check))
        .with_state(state);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap_or(3000)));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn test_endpoint() -> Json<&'static str> {
    println!("📨 Test endpoint llamado");
    Json("test working")
}

fn generate_tasks(params: &MandelbrotParams, regions_per_task: u32) -> Vec<Task> {
    let mut tasks = Vec::new();
    let mut task_id = 1;
    
    // Dividir la imagen en regiones verticales
    let region_height = params.height / regions_per_task;
    
    for i in 0..regions_per_task {
        let y_start = i * region_height;
        let y_end = if i == regions_per_task - 1 {
            params.height  // Última región incluye el resto
        } else {
            (i + 1) * region_height
        };
        
        let region = ImageRegion {
            x_start: 0,
            x_end: params.width,
            y_start,
            y_end,
        };
        
        let task = Task {
            task_id,
            region,
            params: params.clone(),
        };
        
        tasks.push(task);
        task_id += 1;
    }
    
    tasks
}

async fn assign_task(State(state): State<CoordinatorState>) -> Json<serde_json::Value> {
    println!("📨 Coordinator: Recibida solicitud de tarea");
    let mut pending_tasks = state.pending_tasks.lock().unwrap();
    
    if let Some(task) = pending_tasks.pop() {
        println!("📤 Coordinator: Asignando tarea {} (región [{},{}] x [{},{}])", 
            task.task_id,
            task.region.x_start, task.region.x_end,
            task.region.y_start, task.region.y_end);
        
        Json(serde_json::json!({
            "task_id": task.task_id,
            "region": {
                "x_start": task.region.x_start,
                "x_end": task.region.x_end,
                "y_start": task.region.y_start,
                "y_end": task.region.y_end
            },
            "params": {
                "width": task.params.width,
                "height": task.params.height,
                "x_min": task.params.x_min,
                "x_max": task.params.x_max,
                "y_min": task.params.y_min,
                "y_max": task.params.y_max,
                "max_iterations": task.params.max_iterations
            }
        }))
    } else {
        println!("📭 Coordinator: No hay más tareas disponibles");
        Json(serde_json::json!({"error": "No tasks available"}))
    }
}

async fn receive_result(
    State(state): State<CoordinatorState>,
    Json(result): Json<TaskResult>
) -> Json<&'static str> {
    let mut completed_results = state.completed_results.lock().unwrap();
    let total_tasks = *state.total_tasks.lock().unwrap();
    
    completed_results.insert(result.task_id, result.clone());
    
    println!("📥 Coordinator: Recibido resultado de tarea {} del worker {} ({}ms, {} píxeles)", 
        result.task_id, result.worker_id, result.computation_time_ms, result.pixel_data.len());
    
    // Verificar si todas las tareas están completas
    if completed_results.len() as u32 == total_tasks {
        println!("🎉 Coordinator: ¡Todas las {} tareas completadas! Iniciando ensamblaje...", total_tasks);
        assemble_final_image(&completed_results, total_tasks);
    }
    
    Json("OK")
}

async fn get_status(State(state): State<CoordinatorState>) -> Json<serde_json::Value> {
    println!("📊 Coordinator: Recibida solicitud de estado");
    let pending_tasks = state.pending_tasks.lock().unwrap();
    let completed_results = state.completed_results.lock().unwrap();
    let total_tasks = *state.total_tasks.lock().unwrap();
    
    let status = serde_json::json!({
        "total_tasks": total_tasks,
        "pending_tasks": pending_tasks.len(),
        "completed_tasks": completed_results.len(),
        "progress_percentage": (completed_results.len() as f32 / total_tasks as f32) * 100.0
    });
    
    println!("📊 Coordinator: Enviando estado: {:?}", status);
    Json(status)
}

async fn health_check() -> Json<&'static str> {
    Json("healthy")
}

fn assemble_final_image(completed_results: &HashMap<u32, TaskResult>, total_tasks: u32) {
    println!("🖼️ Coordinator: Ensamblando imagen final...");
    
    let mut all_pixels: Vec<u8> = Vec::new();
    let mut total_computation_time = 0u64;
    
    // Ordenar resultados por task_id y combinar píxeles
    let mut sorted_results: Vec<_> = completed_results.values().collect();
    sorted_results.sort_by_key(|r| r.task_id);
    
    for result in sorted_results {
        all_pixels.extend_from_slice(&result.pixel_data);
        total_computation_time += result.computation_time_ms;
    }
    
    println!("✅ Coordinator: Imagen final ensamblada:");
    println!("   - Total píxeles: {}", all_pixels.len());
    println!("   - Tiempo total de cómputo: {}ms", total_computation_time);
    println!("   - Tareas procesadas: {}/{}", completed_results.len(), total_tasks);
    println!("   - Tiempo promedio por tarea: {}ms", total_computation_time / total_tasks as u64);
    
    // TODO: Guardar imagen en archivo .pgm o .png
    println!("💾 Coordinator: Imagen lista para guardar (implementar guardado)");
}