#![no_std]
// #![feature(lang_items)]

use core::panic::PanicInfo;

pub trait Random {
    fn random_num(&self, start: Self, end: Self) -> Self;

    /// Choose a random element from a slice or array
    fn random_choice<'a, T>(&self, slice: &'a [T]) -> &'a T;
}


// Internal LCG function
#[inline(always)]
fn lcg(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005).wrapping_add(1)
}

// Cycle-based seed
#[inline(always)]
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
macro_rules! impl_random_type {
    ($($name:ident => $t:ty),*) => {
        $(
            pub struct $name;

            impl $name {
                #[inline(always)]
                pub fn random_num(start: $t, end: $t) -> $t {
                    let seed = lcg(read_cycles());
                    let range = (end as u64).wrapping_sub(start as u64);
                    start.wrapping_add((seed % range) as $t)
                }

                #[inline(always)]
                pub fn random_choice<'a, T>(slice: &'a [T]) -> &'a T {
                    let len = slice.len() as $t;
                    let idx = Self::random_num(0 as $t, len) as usize;
                    &slice[idx]
                }
            }
        )*
    };
}

// Integer structs
impl_random_type!(
    RandomU8 => u8,
    RandomU16 => u16,
    RandomU32 => u32,
    RandomUsize => usize,
    RandomI8 => i8,
    RandomI16 => i16,
    RandomI32 => i32,
    RandomI64 => i64,
    RandomIsize => isize
);

// Correct macro for floats
macro_rules! impl_random_float_type {
    ($($name:ident => $t:ty),*) => {
        $(
            pub struct $name;

            impl $name {
                #[inline(always)]
                pub fn random_num(start: $t, end: $t) -> $t {
                    let seed = lcg(read_cycles());
                    let fraction = (seed as f64) / (u64::MAX as f64); // [0.0, 1.0)
                    (start as f64 + fraction * ((end - start) as f64)) as $t
                }

                #[inline(always)]
                pub fn random_choice<'a, T>(slice: &'a [T]) -> &'a T {
                    let len = slice.len();
                    let idx = Self::random_num(0.0 as $t, len as $t) as usize;
                    &slice[idx]
                }
            }
        )*
    };
}

// Float structs
impl_random_float_type!(
    RandomF32 => f32,
    RandomF64 => f64
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}