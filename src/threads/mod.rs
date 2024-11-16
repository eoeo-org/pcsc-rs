use arc_swap::ArcSwap;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

use std::{sync::Arc, thread, time::Duration};

use crate::status::SystemStatus;

pub fn spawn_monitor(shared_data: Arc<ArcSwap<SystemStatus>>) {
    let mut sys = System::new();
    let mut disks = Disks::new();

    let builder = thread::Builder::new();

    builder
        .name("System Monitor".into())
        .spawn(move || loop {
            thread::sleep(Duration::from_secs(1));

            sys.refresh_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                    .with_memory(MemoryRefreshKind::everything()),
            );
            disks.refresh_list();
            let current_status = SystemStatus::get(&mut sys, &mut disks);
            shared_data.store(Arc::new(current_status));
        })
        .unwrap();
}
