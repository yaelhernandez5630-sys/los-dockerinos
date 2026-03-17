mod models;
mod coordinator;
mod worker;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Uso: cargo run -- [coordinator | worker]");
        println!("");
        println!("Variables de entorno (Coordinator):");
        println!("  COORDINATOR_PORT=3000");
        println!("  IMAGE_WIDTH=800");
        println!("  IMAGE_HEIGHT=600");
        println!("  REGIONS_PER_TASK=4");
        println!("");
        println!("Variables de entorno (Worker):");
        println!("  COORDINATOR_ENDPOINT=10.10.10.1:3000");
        println!("  WORKER_ID=worker-rogelio-1");
        println!("");
        println!("Ejemplos:");
        println!("  COORDINATOR_PORT=8080 cargo run -- coordinator");
        println!("  COORDINATOR_ENDPOINT=10.10.10.1:8080 WORKER_ID=worker-1 cargo run -- worker");
        return;
    }

    match args[1].as_str() {
        "coordinator" => {
            println!("🎯 Modo Coordinator");
            coordinator::run_coordinator().await;
        }
        "worker" => {
            println!("⚙️ Modo Worker");
            
            // Obtener configuración de variables de entorno
            let coordinator_endpoint = env::var("COORDINATOR_ENDPOINT")
                .unwrap_or_else(|_| {
                    println!("⚠️ Usando COORDINATOR_ENDPOINT por defecto: 10.10.10.1:3000");
                    "10.10.10.1:3000".to_string()
                });
            
            let worker_id = env::var("WORKER_ID")
                .unwrap_or_else(|_| {
                    println!("⚠️ Usando WORKER_ID por defecto: worker-default");
                    "worker-default".to_string()
                });
            
            println!("🔗 Conectando a: {}", coordinator_endpoint);
            println!("🆔 Worker ID: {}", worker_id);
            
            worker::run_worker(&coordinator_endpoint, &worker_id).await;
        }
        _ => {
            println!("❌ Comando no reconocido: {}", args[1]);
            println!("Usa 'coordinator' o 'worker'");
        }
    }
}