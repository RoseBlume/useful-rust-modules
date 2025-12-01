// use std::arch::global_asm;
// #[cfg(target = "i686-pc-windows-mscv")]
// global_asm!{include_str!("../asm/x86/windows/iseven.asm")}
// global_asm!{include_str!("../asm/x86/windows/print.asm")}
// #![no_std]
unsafe extern "C" {
    // #[unsafe(no_mangle)]
    fn q_rsqrt(n: f32) -> f32;
    fn is_asm_even(x: i32) -> i32;
    fn asm_print(msg: *const u8);
}



fn make_printable<'a>(input: &str, buffer: &'a mut [u8]) -> *const u8 {
    let input_bytes = input.as_bytes();
    let max_len = buffer.len().saturating_sub(3); // reserve 3 bytes for \r\n\0

    let copy_len = core::cmp::min(input_bytes.len(), max_len);
    buffer[..copy_len].copy_from_slice(&input_bytes[..copy_len]);

    // append \r\n\0
    buffer[copy_len] = b'\r';
    buffer[copy_len + 1] = b'\n';
    buffer[copy_len + 2] = 0;

    buffer.as_ptr()
}

/// Print a string via MASM `asm_print`
pub fn print(msg: &str) {
    // fixed-size buffer (adjust size as needed)
    let mut buffer = [0u8; 128];

    let ptr = make_printable(msg, &mut buffer);

    unsafe {
        asm_print(ptr); // MASM function expecting *const u8
    }
}


pub fn is_even(x: i32) -> i32 {
    unsafe {
        is_asm_even(x)
    }
}

pub fn inv_sqrt(n: f32) -> f32 {
    unsafe {
        q_rsqrt(n)
    }
}


#[cfg(test)]
mod tests {
    use std::time::Instant;
    use std::ptr::{write_volatile, read_volatile};
    use crate::{inv_sqrt};

    // Prevent compiler from optimizing out the loop
    static mut SINK: f32 = 0.0;
    const LOOPS: u64 = 100_000_000_000;
    // -----------------------------------------------
    // Test 1: MASM fast inverse sqrt
    // -----------------------------------------------
    #[test]
    fn time_masm_qrsqrt() {
        
        let mut x = 1.337_f32;

        println!("Running MASM q_rsqrt {} times...", LOOPS);

        let start = Instant::now();

        for _ in 0..LOOPS {
            let y = inv_sqrt(x);
            unsafe { write_volatile(&raw mut SINK, y); }

            // Make x vary slightly to avoid constant-propagation
            x += 0.000_000_1;
            if x > 2.0 { x = 1.337; }
        }

        let elapsed = start.elapsed();
        let final_value = unsafe { read_volatile(&raw const SINK) };

        println!("MASM q_rsqrt finished in {:?}", elapsed);
        println!("Final SINK value = {}", final_value);
    }

    // -----------------------------------------------
    // Test 2: Rust's standard sqrt
    // -----------------------------------------------
    #[test]
    fn time_rust_sqrt() {
        let mut x = 1.337_f32;

        println!("Running Rust sqrt {} times...", LOOPS);

        let start = Instant::now();

        for _ in 0..LOOPS {
            let y = 1.0_f32 / x.sqrt();
            unsafe { write_volatile(&raw mut SINK, y); }

            x += 0.000_000_1;
            if x > 2.0 { x = 1.337; }
        }

        let elapsed = start.elapsed();
        let final_value = unsafe { read_volatile(&raw const SINK) };

        println!("Rust sqrt finished in {:?}", elapsed);
        println!("Final SINK value = {}", final_value);
    }
}
