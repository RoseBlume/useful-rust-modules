use system_specs::{
    SwapType, 
    Memory,
    CpuInfo,
    DiskInfo
};

static TO_GB: u64 = 1024_u64.pow(3);
static TO_MB: u64 = 1024_u64.pow(2);

fn output_mem_info() {
    let memory = match Memory::get_windows_memory() {
        Some(mem) => mem,
        None => {
            eprintln!("Failed to retrieve memory info");
            return;
        }
    };
    println!("\n=== Memory ===");
    println!("Total RAM: {} MB", (memory.ram.total_free_bytes + memory.ram.total_used_bytes) / TO_MB);
    println!("Used RAM:  {} MB", memory.ram.total_used_bytes / TO_MB);
    println!("Free RAM:  {} MB", memory.ram.total_free_bytes / TO_MB);

    if memory.has_swap() {
        println!("=== Swap ===");
        for (i, swap) in memory.swaps.iter().enumerate() {
            let kind = match swap.kind {
                SwapType::File => "File",
                SwapType::Partition => "Partition",
            };
            println!("Swap{}:", i + 1);
            println!("Swap Type: {}", kind);
            println!("  Used: {} MB", swap.used_bytes / TO_MB);
            println!("  Free: {} MB", swap.free_bytes / TO_MB);
        }
    }
}

fn output_cpu_info() {
    let cpu = CpuInfo::get_cpu_info();
    println!("\n=== CPU ===");
    println!("Model: {}", cpu.model);
    println!("Architecture: {}", cpu.arch);
    println!("Core count: {}", cpu.physical_core_count);
    println!("Total threads: {}", cpu.total_threads);

    for core in &cpu.cores {
        println!(
            "\nCore {}:
    Frequency: {} MHz 
    Threads: {}",
            core.id, core.frequency, core.thread_count
        );
    }
}

fn output_disk_info() {
    println!("\n=== Disks ===");

    let disks = DiskInfo::get_disks();

    for disk in disks {
        println!("Device: {}", disk.label);
        println!("Type:   {}", disk.storage_type);
        println!("Partitions: ");
        for part in disk.partitions {
            println!("    Partition: {}", part.mount_point);
            println!("    Filesystem: {}", part.filesystem);
            println!("    Total:  {} GB", part.total_bytes / TO_GB);
            println!("    Free:   {} GB", part.free_bytes / TO_GB);
        }

        println!();
    }

}

fn main() {
    // Load memory info using our custom module

    // println!("=== CPU ===");
    // // Keeping your existing CPU code (sysinfo)
    // let mut sys = sysinfo::System::new_all();
    // sys.refresh_all();

    // let cpus = sys.cpus();
    // if let Some(cpu) = cpus.first() {
    //     println!("Model: {}", cpu.brand());
    //     println!("Frequency: {} MHz", cpu.frequency());
    // }
    // println!("Architecture: {}", std::env::consts::ARCH);

    output_cpu_info();
    output_mem_info();
    output_disk_info();

    
    println!("=== Notes ===");
    println!("DDR version, RAM speed, and SSD R/W speed cannot be obtained via sysinfo.");
}