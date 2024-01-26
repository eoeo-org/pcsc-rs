use arc_swap::ArcSwap;
use sysinfo::System;

use std::{sync::Arc, thread, time::Duration};

use crate::status::SystemStatus;

pub fn spawn_monitor(shared_data: Arc<ArcSwap<SystemStatus>>) {
    let mut sys = System::new_all();
    let builder = thread::Builder::new();

    builder
        .name("System Monitor".into())
        .spawn(move || loop {
            thread::sleep(Duration::from_secs(1));

            sys.refresh_all();
            let current_status = SystemStatus::get(&mut sys);
            shared_data.store(Arc::new(current_status));
        })
        .unwrap();
}
