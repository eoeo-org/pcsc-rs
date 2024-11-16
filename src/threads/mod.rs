use arc_swap::ArcSwap;
use sysinfo::{Disks, System};

use std::{sync::Arc, thread, time::Duration};

use crate::status::SystemStatus;

pub fn spawn_monitor(shared_data: Arc<ArcSwap<SystemStatus>>) {
    let mut sys = System::new_all();
    let mut disks = Disks::new_with_refreshed_list();

    let builder = thread::Builder::new();

    builder
        .name("System Monitor".into())
        .spawn(move || loop {
            thread::sleep(Duration::from_secs(1));

            sys.refresh_all();
            disks.refresh_list();
            let current_status = SystemStatus::get(&mut sys, &mut disks);
            shared_data.store(Arc::new(current_status));
        })
        .unwrap();
}
