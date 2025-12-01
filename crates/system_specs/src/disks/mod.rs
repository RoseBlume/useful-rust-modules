#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::DiskInfo;



// pub fn get_disks() -> Vec<DiskInfo> {
//     #[cfg(target_os = "linux")]
//     return linux_disks();

//     #[cfg(target_os = "windows")]
//     return windows_disks();
// }

//
// ─────────────────────────────────────────────────────────────────────────────
//   LINUX IMPLEMENTATION
// ─────────────────────────────────────────────────────────────────────────────
//
#[cfg(target_os = "linux")]
pub fn get_disks() -> Vec<DiskInfo> {
    use nix::sys::statvfs::statvfs;

    let mut disks = Vec::new();

    // Read /proc/mounts to get mounted filesystems
    if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let device = parts[0].to_string();
            let mount = parts[1].to_string();

            // Query filesystem stats
            if let Ok(stats) = statvfs(mount.as_str()) {
                let total = stats.blocks() * stats.fragment_size();
                let free = stats.blocks_available() * stats.fragment_size();

                disks.push(DiskInfo {
                    name: device,
                    mount_point: mount,
                    total_bytes: total,
                    free_bytes: free,
                });
            }
        }
    }

    disks
}
