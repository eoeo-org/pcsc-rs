use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    println!("Hello, world!");
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
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

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
