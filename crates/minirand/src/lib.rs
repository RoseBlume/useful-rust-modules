#![no_std]
#[allow(unused_imports)]
use panics;
#[cfg(feature = "cycles")]
mod cycles;
#[cfg(feature = "cycles")]
pub use cycles::{
    RandomF32, RandomF64,
    RandomU8, RandomU16, RandomU32, RandomU64, RandomU128, RandomUsize,
    RandomI8, RandomI16, RandomI32, RandomI64, RandomI128, RandomIsize
};