use crate::models::{Task, TaskResult, HealthResponse};
use std::time::{Duration, Instant};

pub async fn run_worker(coordinator_endpoint: &str, worker_id: &str) {
    println!("⚙️ Iniciando Worker {}... Conectando al Coordinator en {}", worker_id, coordinator_endpoint);
    
    let client = reqwest::Client::new();
    let task_url = format!("http://{}/get_task", coordinator_endpoint);
    let result_url = format!("http://{}/submit_result", coordinator_endpoint);
    let _health_url = format!("http://{}/health", coordinator_endpoint);
    
    let _start_time = Instant::now();
    
    loop {
        // 1. Pedir tarea
        if let Ok(res) = client.get(&task_url).send().await {
            if let Ok(task) = res.json::<Task>().await {
                println!("Worker {}: Recibió tarea {} para región [{},{}] x [{},{}]", 
                    worker_id, task.task_id, 
                    task.region.x_start, task.region.x_end,
                    task.region.y_start, task.region.y_end);
                
                // 2. Calcular Mandelbrot
                let computation_start = Instant::now();
                let pixel_data = calculate_mandelbrot_region(&task);
                let computation_time = computation_start.elapsed().as_millis() as u64;
                
                println!("Worker {}: Cálculo completado en {}ms, {} píxeles generados", 
                    worker_id, computation_time, pixel_data.len());
                
                // 3. Enviar resultado
                let result = TaskResult {
                    task_id: task.task_id,
                    worker_id: worker_id.to_string(),
                    region: task.region,
                    pixel_data,
                    computation_time_ms: computation_time,
                };
                
                if let Ok(_) = client.post(&result_url).json(&result).send().await {
                    println!("Worker {}: Resultado enviado al coordinator", worker_id);
                } else {
                    println!("Worker {}: Error al enviar resultado", worker_id);
                }
            }
        }
        
        // Pausa antes de pedir la siguiente tarea
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

fn calculate_mandelbrot_region(task: &Task) -> Vec<u8> {
    let mut pixel_data = Vec::new();
    let _width = task.region.x_end - task.region.x_start;
    let _height = task.region.y_end - task.region.y_start;
    
    for py in task.region.y_start..task.region.y_end {
        for px in task.region.x_start..task.region.x_end {
            // Convertir píxel a coordenadas del plano complejo
            let x = task.params.x_min + 
                (px as f64 / task.params.width as f64) * (task.params.x_max - task.params.x_min);
            let y = task.params.y_min + 
                (py as f64 / task.params.height as f64) * (task.params.y_max - task.params.y_min);
            
            // Calcular iteraciones de Mandelbrot
            let iterations = mandelbrot_iterations(x, y, task.params.max_iterations);
            
            // Convertir a escala de grises (0-255)
            let gray_value = if iterations == task.params.max_iterations {
                0  // Negro para puntos en el conjunto
            } else {
                (255 * iterations / task.params.max_iterations) as u8
            };
            
            pixel_data.push(gray_value);
        }
    }
    
    pixel_data
}

fn mandelbrot_iterations(x0: f64, y0: f64, max_iterations: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut iteration = 0;
    
    while x * x + y * y <= 4.0 && iteration < max_iterations {
        let x_temp = x * x - y * y + x0;
        y = 2.0 * x * y + y0;
        x = x_temp;
        iteration += 1;
    }
    
    iteration
}

pub async fn health_check(worker_id: &str) -> HealthResponse {
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    HealthResponse {
        status: "healthy".to_string(),
        worker_id: worker_id.to_string(),
        uptime_seconds: uptime,
    }
}