mod status;

use dotenvy::dotenv;
use rust_socketio::{ClientBuilder, Payload, RawClient, TransportType};
use serde_json::json;
use std::{env, process, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};

use crate::status::{CpuData, StatusDataWithPass};

fn main() {
    dotenv().expect(".env file not found");

    let pcsc_uri = match env::var("PCSC_URI") {
        Ok(val) => val,
        Err(_) => "https://pcss.eov2.com/".to_string(),
    };

    if System::IS_SUPPORTED {
        println!("This OS is supported!");
        println!("Hello, world! {}", pcsc_uri);
    } else {
        println!("This OS isn't supported (yet?).");
        process::exit(0x0004);
    }

    /*
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

            println!("=> disks:");
            for disk in sys.disks() {
                println!("{:?}", disk);
            }

            println!("=> system:");
            println!("total memory: {} bytes", sys.total_memory());
            println!("used memory : {} bytes", sys.used_memory());

            println!("{}", json!(system_info));

            for cpu in sys.cpus() {
                println!("{}%", cpu.cpu_usage());
            }
    */

    let hi_event = |payload: Payload, socket: RawClient| {
        match payload {
            Payload::String(str) => println!("Received: {}", str),
            Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
        };

        print!("hi from server");
        init(socket);
    };

    ClientBuilder::new(pcsc_uri)
        .namespace("/server")
        .on("open", |_, _| println!("Connected"))
        .on("close", |_, _| println!("Disconnected"))
        .on("hi", hi_event)
        //.on("sync", send_system_info)
        .on("error", |err, _| eprintln!("Error: {:?}", err))
        .connect()
        .expect("Connection failed");
}

fn init(socket: RawClient) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let _pass = match env::var("PASS") {
        Ok(val) => val,
        Err(_) => "".to_string(),
    };

    let cpu_name = sys.cpus()[0].brand().to_string();
    let os_name = sys.name().expect("Failed to get os name");
    let os_version = sys
        .os_version()
        .or(sys.kernel_version())
        .expect("Failed to get os version");
    let hostname = sys.host_name().expect("Failed to get hostname");

    socket
        .emit(
            "hi",
            json!(StatusDataWithPass {
                pass: _pass,
                _os: format!("{} {}", os_name, os_version),
                hostname: hostname,
                version: "rust".to_string(),
                cpu: CpuData {
                    model: cpu_name,
                    cpus: vec![],
                    percent: 0,
                },
                loadavg: None,
            }),
        )
        .expect("Failed to emit.");
}
