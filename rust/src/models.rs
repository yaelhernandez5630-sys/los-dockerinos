use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub task_id: u32,
    pub command: String,
    // Aquí después irán los rangos de píxeles (ej. y_start, y_end)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResult {
    pub task_id: u32,
    pub worker_id: String,
    pub status: String,
    // Aquí después irá el arreglo de bytes de la imagen calculada
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerInfo {
    pub worker_id: String,
    pub ip_address: String,
}