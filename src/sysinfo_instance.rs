use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

pub struct SysinfoInstance {
    pub system: System,
    pub disks: Disks,
}

impl SysinfoInstance {
    pub fn new() -> SysinfoInstance {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::everything()),
        );
        let disks = Disks::new_with_refreshed_list();

        SysinfoInstance { system, disks }
    }
    pub fn refresh(&mut self) {
        self.system.refresh_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::everything()),
        );
        self.disks.refresh_list();
    }
}
