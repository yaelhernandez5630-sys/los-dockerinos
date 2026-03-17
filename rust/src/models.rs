use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MandelbrotParams {
    pub width: u32,
    pub height: u32,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub max_iterations: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub task_id: u32,
    pub region: ImageRegion,
    pub params: MandelbrotParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageRegion {
    pub x_start: u32,
    pub x_end: u32,
    pub y_start: u32,
    pub y_end: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskResult {
    pub task_id: u32,
    pub worker_id: String,
    pub region: ImageRegion,
    pub pixel_data: Vec<u8>,  // Bytes de la imagen en escala de grises
    pub computation_time_ms: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerInfo {
    pub worker_id: String,
    pub endpoint: String,
    pub status: WorkerStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerStatus {
    Available,
    Busy,
    Offline,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub worker_id: String,
    pub uptime_seconds: u64,
}