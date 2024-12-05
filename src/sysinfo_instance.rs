use sysinfo::{CpuRefreshKind, DiskRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

pub struct SysinfoInstance {
    pub system: System,
    pub disks: Disks,
}

impl SysinfoInstance {
    pub fn new() -> SysinfoInstance {
        let system = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
                .with_memory(MemoryRefreshKind::everything()),
        );
        let disks =
            Disks::new_with_refreshed_list_specifics(DiskRefreshKind::nothing().with_storage());

        SysinfoInstance { system, disks }
    }
    pub fn refresh(&mut self) {
        self.system.refresh_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
                .with_memory(MemoryRefreshKind::everything()),
        );
        self.disks
            .refresh_specifics(true, DiskRefreshKind::nothing().with_storage());
    }
}
