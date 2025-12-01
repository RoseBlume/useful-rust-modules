use std::arch::x86_64::__cpuid;
use std::mem::zeroed;

use std::{thread, time::Duration};
use std::arch::x86_64::_rdtsc;
use std::ptr::null_mut;
// #[derive(Debug)]
// pub struct Cpus {
//     pub cpu_count: i64,
//     pub cpus: Vec<Cpu>,
// }

#[derive(Debug)]
pub struct CpuInfo {
    pub model: String,
    pub arch: String,
    pub physical_core_count: i64,
    pub cores: Vec<Core>,
    pub total_threads: i128,
}

#[derive(Debug)]
pub struct Core {
    pub id: i64,
    pub frequency: u64,
    pub thread_count: i8,
}

#[repr(C)]
struct SYSTEM_INFO {
    w_processor_architecture: u16,
    w_reserved: u16,
    dw_page_size: u32,
    lp_min_app_addr: *mut u8,
    lp_max_app_addr: *mut u8,
    dw_active_processor_mask: usize,
    dw_number_of_processors: u32,
    dw_processor_type: u32,
    dw_allocation_granularity: u32,
    w_processor_level: u16,
    w_processor_revision: u16,
}

unsafe extern "system" {
    fn GetSystemInfo(lpSystemInfo: *mut SYSTEM_INFO);
}

impl CpuInfo {
    pub fn get_cpu_info() -> CpuInfo {
        // Get core count from Windows
        let cores = Self::get_cores();
        let mut sys_info: SYSTEM_INFO = unsafe { zeroed() };
        unsafe { GetSystemInfo(&mut sys_info as *mut SYSTEM_INFO) };

        let total_threads = sys_info.dw_number_of_processors as i128;
        let core_count = Self::get_num_physical_cores(); // logical cores; physical cores need more complex API

        let model = Self::get_cpu_model();

        // Fill cores
        // let mut cores = Vec::new();
        // for i in 0..core_count {
        //     cores.push(Core {
        //         id: i as i64,
        //         model: model.clone(),
        //         frequency: Self::get_cpu_frequency(),
        //         thread_count: 1, // logical per core
        //     });
        // }

        CpuInfo {
            model,
            arch: std::env::consts::ARCH.to_string(),
            physical_core_count: core_count as i64,
            cores,
            total_threads,
        }
    }

    fn get_cpu_model() -> String {
        let mut brand = Vec::with_capacity(48);

        unsafe {
            for i in 0x80000002..=0x80000004 {
                let res = __cpuid(i);
                brand.extend_from_slice(&res.eax.to_le_bytes());
                brand.extend_from_slice(&res.ebx.to_le_bytes());
                brand.extend_from_slice(&res.ecx.to_le_bytes());
                brand.extend_from_slice(&res.edx.to_le_bytes());
            }
        }

        String::from_utf8_lossy(&brand).trim().to_string()
    }

    fn get_num_physical_cores() -> u64 {
    // Inspired by https://msdn.microsoft.com/en-us/library/ms683194


        #[allow(non_upper_case_globals)]
        const RELATION_PROCESSOR_CORE: u32 = 0;

        #[repr(C)]
        #[allow(non_camel_case_types)]
        struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
            mask: usize,
            relationship: u32,
            _unused: [u64; 2]
        }

        unsafe extern "system" {
            fn GetLogicalProcessorInformation(
                info: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION,
                length: &mut u32
            ) -> u32;
        }

        // First we need to determine how much space to reserve.

        // The required size of the buffer, in bytes.
        let mut needed_size = 0;

        unsafe {
            GetLogicalProcessorInformation(null_mut(), &mut needed_size);
        }

        let struct_size = size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>() as u32;

        // Could be 0, or some other bogus size.
        if needed_size == 0 || needed_size < struct_size || needed_size % struct_size != 0 {
            panic!("Got error");
        }

        let count = needed_size / struct_size;

        // Allocate some memory where we will store the processor info.
        let mut buf = Vec::with_capacity(count as usize);

        let result;

        unsafe {
            result = GetLogicalProcessorInformation(buf.as_mut_ptr(), &mut needed_size);
        }

        // Failed for any reason.
        if result == 0 {
            panic!("Got result of 0");
        }

        let count = needed_size / struct_size;

        unsafe {
            buf.set_len(count as usize);
        }

        let phys_proc_count = buf.iter()
            // Only interested in processor packages (physical processors.)
            .filter(|proc_info| proc_info.relationship == RELATION_PROCESSOR_CORE)
            .count();

        phys_proc_count as u64
    }

    fn get_cpu_frequency() -> u64 {
        const SLEEP_MS: u64 = 100;

        unsafe {
            let start = _rdtsc();
            thread::sleep(Duration::from_millis(SLEEP_MS));
            let end = _rdtsc();

            let cycles = end - start;
            let frequency_hz = cycles * 1000 / SLEEP_MS; // scale to per second
            frequency_hz / 1_000_000 // return MHz
        }
    }
    pub fn get_cores() -> Vec<Core> {
        // let model = Self::get_cpu_model();

        #[repr(C)]
        struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
            relationship: u32,
            size: u32,
            // Followed by union (we will read manually)
        }

        #[repr(C)]
        struct PROCESSOR_CORE {
            flags: u8,
            reserved: [u8; 21],
            group_count: u16,
            group_mask: usize, // simplified for 1 group
        }

        const RELATION_PROCESSOR_CORE: u32 = 0;

        unsafe extern "system" {
            fn GetLogicalProcessorInformationEx(
                relationship_type: u32,
                buffer: *mut u8,
                returned_length: *mut u32,
            ) -> i32;
        }

        // First, determine required buffer size
        let mut size: u32 = 0;
        unsafe {
            GetLogicalProcessorInformationEx(RELATION_PROCESSOR_CORE, null_mut(), &mut size);
        }

        let mut buffer: Vec<u8> = vec![0; size as usize];

        let success = unsafe {
            GetLogicalProcessorInformationEx(
                RELATION_PROCESSOR_CORE,
                buffer.as_mut_ptr(),
                &mut size,
            )
        };

        if success == 0 {
            panic!("Failed to get processor information");
        }

        // Walk the buffer to extract each core's logical processor mask
        let mut cores = Vec::new();
        let mut offset = 0;

        while offset < size as usize {
            let info_ptr = unsafe {
                buffer.as_ptr().add(offset) as *const SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX
            };
            let info = unsafe { &*info_ptr };

            if info.relationship == RELATION_PROCESSOR_CORE {
                // Processor core data starts immediately after SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX
                let core_ptr = unsafe { (info_ptr as *const u8).add(size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX>()) as *const PROCESSOR_CORE };
                let core = unsafe { &*core_ptr };

                // Count logical processors in mask
                let thread_count = core.group_mask.count_ones() as i8;

                cores.push(Core {
                    id: cores.len() as i64,
                    frequency: Self::get_cpu_frequency(),
                    thread_count,
                });
            }

            offset += info.size as usize;
        }

        cores
    }
}

