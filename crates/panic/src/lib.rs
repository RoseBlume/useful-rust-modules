#![no_std]
#[cfg(target_os = "windows")]
mod windows;

// #[cfg(target_os = "windows")]
// pub use windows::panic;

#[cfg(not(target_os = "windows"))]
mod linux;

// #[cfg(target_os = "linux")]
// pub use linux::panic;