mod models;
mod coordinator;
mod worker;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Uso: cargo run -- [coordinator | worker <HUB_IP> <WORKER_ID>]");
        return;
    }

    match args[1].as_str() {
        "coordinator" => {
            coordinator::run_coordinator().await;
        }
        "worker" => {
            if args.len() < 4 {
                println!("Faltan argumentos. Uso: cargo run -- worker 10.10.10.1 Worker-Gabriel");
                return;
            }
            worker::run_worker(&args[2], &args[3]).await;
        }
        _ => println!("Comando no reconocido."),
    }
}