use rust_socketio::{ClientBuilder, Event, Payload, RawClient};
use serde_json::json;
use std::{thread, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};

use crate::status::{CpuData, StatusData, StatusDataWithPass};

pub fn main() {
    let mut sys = System::new_all();

    thread::spawn(move || loop {
        sys.refresh_all();

        let cpu_name = sys.cpus()[0].brand().to_string();
        let os_name = sys.name().expect("Failed to get os name");
        let os_version = sys
            .os_version()
            .or(sys.kernel_version())
            .expect("Failed to get os version");
        let hostname = sys.host_name().expect("Failed to get hostname");

        let load_avg = sys.load_average();
        let loadavg: Option<[f64; 3]> = match os_name.as_str() {
            "Windows" => None,
            _ => Some([load_avg.one, load_avg.five, load_avg.fifteen]),
        };

        println!("System OS: {} {}", os_name, os_version);

        println!("=> disks:");
        for disk in sys.disks() {
            println!("{:?}", disk);
        }

        println!("=> system:");
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());

        for cpu in sys.cpus() {
            println!("{}%", cpu.cpu_usage());
        }

        let system_info = StatusData {
            _os: format!("{} {}", os_name.clone(), os_version.clone()),
            hostname: hostname.clone(),
            version: "rust".to_string(),
            cpu: CpuData {
                model: cpu_name.clone(),
                cpus: vec![],
                percent: 0,
            },
            loadavg: loadavg.clone(),
        };

        println!("{}", json!(system_info));

        thread::sleep(Duration::from_secs(1));
    });
}
