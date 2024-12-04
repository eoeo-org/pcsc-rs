use arc_swap::ArcSwap;

use std::{sync::Arc, thread, time::Duration};

use crate::{status::SystemStatus, sysinfo_instance::SysinfoInstance};

pub fn spawn_monitor(shared_data: Arc<ArcSwap<SystemStatus>>) {
    let mut sys = SysinfoInstance::new();

    let builder = thread::Builder::new();

    builder
        .name("System Monitor".into())
        .spawn(move || loop {
            thread::sleep(Duration::from_secs(1));

            sys.refresh();
            let current_status = SystemStatus::get(&mut sys);
            shared_data.store(Arc::new(current_status));
        })
        .unwrap();
}
