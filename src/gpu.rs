use std::process::{Command, Output};
use crate::status::{GpuData, GpuMemory};

pub fn get_info() -> Output {
    Command::new("nvidia-smi")
        .args(["--format=csv", "--query-gpu=name,utilization.gpu,memory.free,memory.total"])
        .output()
        .expect("failed to execute process")
}
