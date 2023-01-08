use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CpuCoreUsage {
  pub(crate) cpu: i16
}

#[derive(Serialize, Deserialize)]
pub struct CpuData {
  pub(crate) model: String,
  pub(crate) cpus: Vec<CpuCoreUsage>,
  pub(crate) percent: i16
}

#[derive(Serialize, Deserialize)]
pub struct StatusData {
  pub(crate) _os: String,
  pub(crate) hostname: String,
  pub(crate) version: String,
  pub(crate) cpu: CpuData
}
