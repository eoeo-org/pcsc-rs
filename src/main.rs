mod status;

use dotenvy::dotenv;
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::json;
use std::env;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};

use crate::status::{CpuData, CpuCoreUsage, StatusData};

fn main() {
    dotenv().expect(".env file not found");

    /*for (key, value) in env::vars() {
        println!("{key}: {value}");
    }*/

    println!("Hello, world!");

    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_name = sys.cpus()[0].brand().to_string();
    let os_name = sys.name().expect("Failed to get os name");
    let os_version = sys.os_version().expect("Failed to get os version");
    let hostname = sys.host_name().expect("Failed to get hostname");

    println!("System OS: {} {}", os_name, os_version);

    let mut system_info = StatusData {
        _os: format!("{} {}", os_name, os_version),
        hostname: hostname,
        version: "".to_string(),
        cpu: CpuData {
            model: "".to_string(),
            cpus: vec![],
            percent: 0,
        },
    };

    println!("{}", json!(system_info));

    let send_system_info = |payload: Payload, socket: RawClient| match payload {
        Payload::String(str) => {
            println!("Received: {}", str);
            //socket.emit("test", json!(system_info)).expect("Failed to emit.");
        }
        _ => {}
    };

    let socket = ClientBuilder::new("http://localhost:4200")
        .namespace("/")
        .on("connect", |_, _| println!("Connected"))
        .on("disconnect", |_, _| println!("Disconnected"))
        .on("hi", send_system_info)
        .on("sync", send_system_info)
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");

    let json_payload = json!({"token": 123});
    socket
        .emit("foo", json_payload)
        .expect("Server unreachable");

    // Update all information

    let load_avg = sys.load_average();
    println!(
        "one minute: {}, five minutes: {}, fifteen minutes: {}",
        load_avg.one, load_avg.five, load_avg.fifteen,
    );

    // Disks
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    println!("=> system:");
    // RAM and swap:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // System information:
    println!(
        "System host name: {}",
        sys.host_name().expect("Failed to get hostname")
    );

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    // CPU Usage
    loop {
        sys.refresh_cpu(); // Refreshing CPU information.
        println!("---------------");
        //println!("{}", sys.cpus());
        for cpu in sys.cpus() {
            println!("{}%", cpu.cpu_usage());
        }
        println!("---------------");
        std::thread::sleep(Duration::from_secs(1));
    }
}
