use std::mem::zeroed;


pub struct Ram {
    pub total_free_bytes: u64,
    pub total_used_bytes: u64,
    pub _module_count: u16,
    pub _modules: Vec<Modules>,
}

pub struct Modules {
    pub name: String,
    pub ddr_version: String,
    pub frequency_mhz: u64,
    pub free_bytes: u64,
    pub used_bytes: u64,
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

impl Ram {
    pub fn get_ram() -> Option<Ram> {
        unsafe {
            let mut mem_status: MEMORYSTATUSEX = zeroed();
            mem_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

            if GlobalMemoryStatusEx(&mut mem_status as *mut MEMORYSTATUSEX) != 0 {
                let total_used_bytes = mem_status.ullTotalPhys - mem_status.ullAvailPhys;
                Some(Ram {
                    total_free_bytes: mem_status.ullAvailPhys,
                    total_used_bytes,
                    _module_count: 0,       // Cannot get this without WMI
                    _modules: Vec::new(),   // Cannot get module details without WMI
                })
            } else {
                None
            }
        }
    }
}

