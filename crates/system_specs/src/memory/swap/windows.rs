use std::mem::zeroed;

#[derive(Debug)]
pub struct SwapDevice {
    pub kind: SwapType,
    pub free_bytes: u64,
    pub used_bytes: u64,
}

#[derive(Debug)]
pub enum SwapType {
    File,       // In Windows, pagefile.sys is the swap file
    Partition,  // Placeholder, Windows doesn't expose separate swap partitions easily
}

#[repr(C)]
#[allow(non_snake_case)]
struct MEMORYSTATUSEX {
    dwLength: u32,
    dwMemoryLoad: u32,
    ullTotalPhys: u64,
    ullAvailPhys: u64,
    ullTotalPageFile: u64,
    ullAvailPageFile: u64,
    ullTotalVirtual: u64,
    ullAvailVirtual: u64,
    ullAvailExtendedVirtual: u64,
}

unsafe extern "system" {
    fn GlobalMemoryStatusEx(lpBuffer: *mut MEMORYSTATUSEX) -> i32;
}

impl SwapDevice {
    pub fn get_swap() -> Option<SwapDevice> {
        unsafe {
            let mut mem_status: MEMORYSTATUSEX = zeroed();
            mem_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

            if GlobalMemoryStatusEx(&mut mem_status as *mut MEMORYSTATUSEX) != 0 {
                let used_bytes = mem_status.ullTotalPageFile - mem_status.ullAvailPageFile;
                Some(SwapDevice {
                    kind: SwapType::File,
                    free_bytes: mem_status.ullAvailPageFile,
                    used_bytes,
                })
            } else {
                None
            }
        }
    }
}