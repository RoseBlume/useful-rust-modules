// Internal LCG function
pub fn lcg(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005).wrapping_add(1)
}

// Cycle-based seed
pub fn read_cycles() -> u64 {
    #[cfg(target_arch = "aarch64")]
    {
        let value: u64;
        unsafe {
            core::arch::asm!(
                "mrs {0}, cntvct_el0",
                out(reg) value,
                options(nomem, nostack, preserves_flags)
            );
        }
        return value;
    }

    #[cfg(all(target_arch = "arm", target_os = "linux"))]
    {
        let low: u32;
        let high: u32;
        unsafe {
            core::arch::asm!(
                "mrrc p15, 0, {0}, {1}, c14",
                out(reg) low,
                out(reg) high,
                options(nomem, nostack, preserves_flags)
            );
        }
        return ((high as u64) << 32) | (low as u64);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        let low: u32;
        let high: u32;
        unsafe {
            core::arch::asm!(
                "rdtsc",
                out("eax") low,
                out("edx") high,
                options(nomem, nostack, preserves_flags)
            );
        }
        return ((high as u64) << 32) | (low as u64);
    }

    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    {
        // Simple pseudo-cycle estimate using addresses
        let a = &0 as *const _ as u64;
        let b = &read_cycles as *const () as u64;
        let mut x = a ^ (b.rotate_left(13));
        x ^= x >> 7;
        x ^= x << 9;
        x ^= x >> 13;
        return x;
    }

    #[cfg(not(any(
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "wasm32",
        target_arch = "wasm64"
    )))]
    compile_error!("Unsupported architecture for cycle-based RNG");
}