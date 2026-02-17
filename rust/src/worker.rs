use crate::models::{Task, TaskResult};
use std::time::Duration;

pub async fn run_worker(hub_ip: &str, worker_id: &str) {
    println!("⚙️ Iniciando Worker {}... Conectando al Hub en {}", worker_id, hub_ip);
    let client = reqwest::Client::new();
    let task_url = format!("http://{}:3000/get_task", hub_ip);
    let result_url = format!("http://{}:3000/submit_result", hub_ip);

    loop {
        // 1. Pedir tarea
        if let Ok(res) = client.get(&task_url).send().await {
            if let Ok(task) = res.json::<Task>().await {
                println!("Worker {}: Recibió tarea {}. Comando: {}", worker_id, task.task_id, task.command);
                
                // 2. Simular cómputo pesado
                tokio::time::sleep(Duration::from_secs(2)).await;
                
                // 3. Enviar resultado
                let result = TaskResult {
                    task_id: task.task_id,
                    worker_id: worker_id.to_string(),
                    status: "COMPLETADO_CON_EXITO".to_string(),
                };
                
                let _ = client.post(&result_url).json(&result).send().await;
                println!("Worker {}: Resultado enviado al coordinador.", worker_id);
            }
        }
        
        // Pausa antes de pedir la siguiente tarea
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}