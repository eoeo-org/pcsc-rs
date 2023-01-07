use dotenvy::dotenv;
use std::env;
use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    dotenv().expect(".env file not found");

    println!("Hello, world!");

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    let mut sys = System::new_all();

    // Update all information
    sys.refresh_all();

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
    let os_name = sys.name().expect("Failed to get os name");
    let os_version = sys.os_version().expect("Failed to get os version");
    
    println!("System OS: {} {}", os_name, os_version);
    println!("System host name: {}", sys.host_name().expect("Failed to get hostname"));

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
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
