use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CpuCoreUsage {
    pub(crate) cpu: i16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CpuData {
    pub(crate) model: String,
    pub(crate) cpus: Vec<CpuCoreUsage>,
    pub(crate) percent: i16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StatusData {
    pub(crate) _os: String,
    pub(crate) hostname: String,
    pub(crate) version: String,
    pub(crate) cpu: CpuData,
    pub(crate) loadavg: Option<[f64; 3]>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StatusDataWithPass {
    pub(crate) pass: String,
    pub(crate) _os: String,
    pub(crate) hostname: String,
    pub(crate) version: String,
    pub(crate) cpu: CpuData,
    pub(crate) loadavg: Option<[f64; 3]>,
}
