use std::time::Instant;
use std::ptr::{write_volatile, read_volatile};
use multi_asm::{inv_sqrt};

// Prevent compiler from optimizing out the loop
static mut SINK: f32 = 0.0;
const LOOPS: u64 = 100_000_000;

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

fn main() {
    time_masm_qrsqrt();
    time_rust_sqrt();

}