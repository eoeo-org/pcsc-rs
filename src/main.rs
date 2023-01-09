mod status;

use dotenvy::dotenv;
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::json;
use std::time::Duration;
use std::process;
use std::{any::Any, env};
use sysinfo::{CpuExt, System, SystemExt};

use crate::status::{CpuCoreUsage, CpuData, StatusData, StatusDataWithPass};

fn main() {
    dotenv().expect(".env file not found");

    let PASS = match env::var("PASS") {
        Ok(val) => val,
        Err(_) => "".to_string(),
    };

    let PCSC_URI = match env::var("PCSC_URI") {
        Ok(val) => val,
        Err(_) => "https://pcss.eov2.com/".to_string(),
    };

    if System::IS_SUPPORTED {
        println!("This OS is supported!");
        println!("Hello, world! {}", PCSC_URI);
    } else {
        println!("This OS isn't supported (yet?).");
        process::exit(0x0004);
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_name = sys.cpus()[0].brand().to_string();
    let os_name = sys.name().expect("Failed to get os name");
    let os_version = sys.os_version().or(sys.kernel_version()).expect("Failed to get os version");
    let hostname = sys.host_name().expect("Failed to get hostname");

    let data = StatusDataWithPass {
        pass: PASS.clone(),
        _os: format!("{} {}", os_name.clone(), os_version.clone()),
        hostname: hostname.clone(),
        version: "rust".to_string(),
        cpu: CpuData {
            model: cpu_name.clone(),
            cpus: vec![],
            percent: 0,
        },
        loadavg: None,
    };

    println!("{}", json!(data));

    let send_system_info = move |payload: Payload, socket: RawClient| {
        match payload {
            Payload::String(str) => println!("Received: {}", str),
            Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
        };

        print!("hi from server");

        socket
            .emit("hi", json!(&data.clone()))
            .expect("Failed to emit.");

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
                //println!("{}%", cpu.cpu_usage());
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    };

    let socket = ClientBuilder::new(PCSC_URI)
        .namespace("/server")
        .on("connect", |_, _| println!("Connected"))
        .on("disconnect", |_, _| println!("Disconnected"))
        .on("hi", send_system_info)
        //.on("sync", send_system_info)
        .on("error", |err, _| eprintln!("Error: {:#?}", err));

    socket.connect().expect("Connection failed");
}
