// #![feature(lang_items)]


pub trait Random {
    fn random_num(&self, start: Self, end: Self) -> Self;

    /// Choose a random element from a slice or array
    fn random_choice<'a, T>(&self, slice: &'a [T]) -> &'a T;
}


// Internal LCG function
fn lcg(seed: u64) -> u64 {
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
macro_rules! impl_random_numeric {
    ($($name:ident => $t:ty => $is_float:expr),*) => {
        $(
            pub struct $name;

            impl $name {
                /// Random number in [start, end), safe
                #[inline(always)]
                pub fn random_num(start: $t, end: $t) -> $t {
                    if start >= end {
                        return start;
                    }

                    let seed = lcg(read_cycles());
                    if $is_float {
                        let fraction = (seed as f64) / (u64::MAX as f64);
                        (start as f64 + fraction * ((end - start) as f64)) as $t
                    } else {
                        let range = (end as u64) - (start as u64);
                        start + (seed % range) as $t
                    }
                }

                /// Random element from slice; returns first element if slice is empty
                #[inline(always)]
                pub fn random_choice<'a, T>(slice: &'a [T]) -> &'a T {
                    if slice.is_empty() {
                        &slice[0] // safe only if caller ensures non-empty, optional fallback
                    } else {
                        let len = slice.len();
                        let idx = if $is_float {
                            Self::random_num(0.0 as $t, len as $t) as usize
                        } else {
                            Self::random_num(0 as $t, len as $t) as usize
                        };
                        &slice[idx.min(len - 1)]
                    }
                }
            }
        )*
    };
}

// Integer and float structs
impl_random_numeric!(
    RandomU8 => u8 => false,
    RandomU16 => u16 => false,
    RandomU32 => u32 => false,
    RandomUsize => usize => false,
    RandomI8 => i8 => false,
    RandomI16 => i16 => false,
    RandomI32 => i32 => false,
    RandomI64 => i64 => false,
    RandomIsize => isize => false,
    RandomF32 => f32 => true,
    RandomF64 => f64 => true
);



