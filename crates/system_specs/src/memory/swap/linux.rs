use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct SwapDevice {
    pub kind: SwapType,
    pub free_bytes: u64,
    pub used_bytes: u64,
}

#[derive(Debug)]
pub enum SwapType {
    File,
    Partition,
}

impl SwapDevice {
    pub fn get_swap() -> Option<Vec<SwapDevice>> {
        let swaps_path = Path::new("/proc/swaps");
        if !swaps_path.exists() {
            return None;
        }

        let file = File::open(swaps_path).ok()?;
        let reader = BufReader::new(file);

        let mut devices = Vec::new();

        // /proc/swaps format:
        // Filename                Type        Size    Used    Priority
        // /swapfile               file        2097148 0       -2
        for line in reader.lines().skip(1) { // skip header
            if let Ok(line) = line {
                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() < 5 {
                    continue;
                }

                let path = parts[0];
                let kind = match parts[1].to_lowercase().as_str() {
                    "file" => SwapType::File,
                    "partition" => SwapType::Partition,
                    _ => continue,
                };

                let size_kb: u64 = parts[2].parse().unwrap_or(0);
                let used_kb: u64 = parts[3].parse().unwrap_or(0);
                let free_kb = size_kb.saturating_sub(used_kb);

                devices.push(SwapDevice {
                    kind,
                    free_bytes: free_kb * 1024,
                    used_bytes: used_kb * 1024,
                });
            }
        }

        Some(devices)
    }

    /// Optional helper: total swap in bytes
    pub fn get_total_swap_bytes() -> Option<(u64, u64)> {
        let devices = Self::get_swap()?;
        let mut total_free = 0;
        let mut total_used = 0;
        for dev in devices {
            total_free += dev.free_bytes;
            total_used += dev.used_bytes;
        }
        Some((total_free, total_used))
    }
}
