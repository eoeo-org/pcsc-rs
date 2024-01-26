use std::env;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use sysinfo::{
    Cpu, System, Disks
};

use crate::{gpu, unix_to_date};

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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RamData {
    pub(crate) free: u64,
    pub(crate) total: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SwapData {
    pub(crate) free: u64,
    pub(crate) total: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageData {
    pub(crate) name: String,
    pub(crate) free: u64,
    pub(crate) total: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GpuMemory {
    pub(crate) free: u64,
    pub(crate) total: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GpuData {
    pub(crate) name: String,
    pub(crate) usage: Option<u64>,
    pub(crate) memory: GpuMemory,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SystemStatus {
    pub(crate) _os: String,
    pub(crate) hostname: String,
    pub(crate) version: String,
    pub(crate) cpu: CpuData,
    pub(crate) ram: RamData,
    pub(crate) swap: SwapData,
    pub(crate) storages: Vec<StorageData>,
    #[serde(rename = "loadavg")]
    pub(crate) load_average: Option<[f64; 3]>,
    pub(crate) uptime: String,
    pub(crate) gpu: Option<GpuData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StatusDataWithPass {
    #[serde(flatten)]
    pub(crate) status: SystemStatus,
    pub(crate) pass: String,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

impl SystemStatus {
    pub fn get(sys: &mut System) -> Self {
        let cpu_name = sys.cpus()[0].brand().to_string();
        let os_name = System::name().expect("Failed to get os name");
        let os_version = System::os_version()
            .or(System::kernel_version())
            .expect("Failed to get os version");

        let hostname = env::var("HOSTNAME").unwrap_or_else(|_| System::host_name().expect("Failed to get hostname"));

        cfg_if! {
            if #[cfg(target_os = "windows")] {
                let load_average = None;
            } else {
                let load_average = System::load_average();
                let load_average = Some([load_average.one, load_average.five, load_average.fifteen]);
            }
        };

        let cpus: Vec<CoreData> = sys.cpus().iter().map(Into::into).collect();

        let cpu = CpuData {
            model: cpu_name.clone(),
            cpus,
        };

        let ram = RamData {
            free: sys.available_memory(),
            total: sys.total_memory(),
        };

        let swap = SwapData {
            free: sys.free_swap(),
            total: sys.total_swap(),
        };

        let uptime = unix_to_date::new(System::uptime());

        let disks = Disks::new_with_refreshed_list();

        let storages: Vec<StorageData> = disks
            .iter()
            .map(|disk| StorageData {
                name: disk.name().to_string_lossy().to_string(),
                free: disk.available_space(),
                total: disk.total_space(),
            })
            .collect();

        let gpu = gpu::get_info();

        Self {
            _os: format!("{} {}", os_name.clone(), os_version.clone()),
            version: format!("Rust client v{}", VERSION.to_string()),
            hostname,
            cpu,
            ram,
            swap,
            load_average,
            uptime,
            storages,
            gpu,
        }
    }

    pub fn with_pass(self, pass: String) -> StatusDataWithPass {
        StatusDataWithPass { status: self, pass }
    }
}
