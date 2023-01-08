mod status;

use dotenvy::dotenv;
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::json;
use std::{env, any::Any};
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};

use crate::status::{CpuCoreUsage, CpuData, StatusData};

fn main() {
    dotenv().expect(".env file not found");

    let PASS = match env::var("PASS") {
        Ok(val) => val,
        Err(_) => "".to_string(),
    };

    let PCSC_URI = match env::var("PCSC_URI") {
        Ok(val) => val,
        Err(_) => "https://pcss.eov2.com".to_string(),
    };

    println!("Hello, world! {}", PCSC_URI);

    let mut sys = System::new_all();

    let send_system_info = |payload: Payload, socket: RawClient| match payload {
        Payload::String(str) => {
            println!("Received: {}", str);
            //socket.emit("test", json!(system_info)).expect("Failed to emit.");
        }
        _ => {}
    };

    let socket = ClientBuilder::new(PCSC_URI)
        .namespace("/server")
        .on("connect", |_, _| println!("Connected"))
        .on("disconnect", |_, _| println!("Disconnected"))
        .on("hi", send_system_info)
        .on("sync", send_system_info)
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");

    /*     let json_payload = json!({"token": 123});
    socket
        .emit("foo", json_payload)
        .expect("Server unreachable");
     */

    loop {
        sys.refresh_all();

        let cpu_name = sys.cpus()[0].brand().to_string();
        let os_name = sys.name().expect("Failed to get os name");
        let os_version = sys.os_version().expect("Failed to get os version");
        let hostname = sys.host_name().expect("Failed to get hostname");

        let load_avg = sys.load_average();
        let loadavg: Option<[f64; 3]> = match os_name.as_str() {
            "Windows" => None,
            _ => Some([load_avg.one, load_avg.five, load_avg.fifteen]),
        };

        println!("System OS: {} {}", os_name, os_version);

        let mut system_info = StatusData {
            _os: format!("{} {}", os_name, os_version),
            hostname: hostname,
            version: "rust".to_string(),
            cpu: CpuData {
                model: cpu_name.to_string(),
                cpus: vec![],
                percent: 0,
            },
            loadavg,
        };

        println!("=> disks:");
        for disk in sys.disks() {
            println!("{:?}", disk);
        }

        println!("=> system:");
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());

        println!("{}", json!(system_info));

        for cpu in sys.cpus() {
            //println!("{}%", cpu.cpu_usage());
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
