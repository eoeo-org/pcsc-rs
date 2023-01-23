use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use sysinfo::{Cpu, CpuExt, System, SystemExt};
use std::{env, path::{Component}};

use crate::unix_to_date;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CoreData {
    #[serde(rename = "cpu")]
    pub(crate) usage: f32,
}
impl From<&Cpu> for CoreData {
    fn from(value: &Cpu) -> Self {
        Self {
            usage: value.cpu_usage(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CpuData {
    pub(crate) model: String,
    pub(crate) cpus: Vec<CoreData>,
    #[serde(rename = "percent")]
    pub(crate) average_usage: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RamData {
    pub(crate) free: u64,
    pub(crate) total: u64,
    #[serde(rename = "percent")]
    pub(crate) usage: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageData {
    pub(crate) free: u64,
    pub(crate) total: u64,
    #[serde(rename = "percent")]
    pub(crate) usage: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SystemStatus {
    pub(crate) _os: String,
    pub(crate) hostname: String,
    pub(crate) version: String,
    pub(crate) cpu: CpuData,
    pub(crate) ram: RamData,
    pub(crate) storage: StorageData,
    #[serde(rename = "loadavg")]
    pub(crate) load_average: Option<[f64; 3]>,
    pub(crate) uptime: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StatusDataWithPass {
    #[serde(flatten)]
    pub(crate) status: SystemStatus,
    pub(crate) pass: String,
}

impl SystemStatus {
    pub fn get(sys: &mut System) -> Self {
        let cpu_name = sys.cpus()[0].brand().to_string();
        let os_name = sys.name().expect("Failed to get os name");
        let os_version = sys
            .os_version()
            .or(sys.kernel_version())
            .expect("Failed to get os version");

        let hostname = sys.host_name().expect("Failed to get hostname");

        cfg_if! {
            if #[cfg(target_os = "windows")] {
                let load_average = None;
            } else {
                let load_average = sys.load_average();
                let load_average = Some([load_average.one, load_average.five, load_average.fifteen]);
            }
        };

        //println!("=> disk:");
        //println!("{:?}", sys.disks().iter().position(|&disk|));

        //println!("=> system:");
        //println!("total memory: {} bytes", sys.total_memory());
        //println!("used memory : {} bytes", sys.used_memory());

        let cpus: Vec<CoreData> = sys.cpus().iter().map(Into::into).collect();

        let cpu_percent = cpus.iter().map(|core| core.usage).sum::<f32>() / cpus.len() as f32;
        let cpu_percent = cpu_percent.round();

        let cpu = CpuData {
            model: cpu_name.clone(),
            cpus,
            average_usage: cpu_percent,
        };

        let ram = RamData {
            free: sys.free_memory(),
            total: sys.total_memory(),
            usage: {
                let free = sys.free_memory() as f32 / sys.total_memory() as f32;
                ((1.0 - free) * 100.0).ceil()
            },
        };

        let uptime = unix_to_date::new(sys.uptime());

        /*for disk in sys.disks() {
            println!("{}/{}", disk.available_space(), disk.total_space());
        }*/


        let dir = env::current_dir().unwrap();
        for disk in dir.components() {
            println!("{:?}", disk.as_os_str());
        }
        let disk = sys.disks().iter().next();

        Self {
            _os: format!("{} {}", os_name.clone(), os_version.clone()),
            version: "Rust".into(),
            hostname,
            cpu,
            ram,
            load_average,
            uptime,
            storage: StorageData { free: 0, total: 0, usage: 0. },
        }
    }

    pub fn with_pass(self, pass: String) -> StatusDataWithPass {
        StatusDataWithPass { status: self, pass }
    }
}
