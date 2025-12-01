mod ram;
mod swap;
use ram::Ram;
use swap::SwapDevice;
pub use swap::SwapType;


pub struct Memory {
    pub ram: Ram,
    pub swaps: Vec<SwapDevice>,
}

impl Memory {
    /// Retrieve system memory info (RAM + swap) for Windows
    pub fn get_windows_memory() -> Option<Memory> {
        // Get RAM info
        let ram = Ram::get_ram()?;

        // Get swap info
        let swap = match SwapDevice::get_swap() {
            Some(s) => vec![s],
            None => Vec::new(),
        };

        Some(Memory {
            ram,
            swaps: swap,
        })
    }


    /// Check if system has any swap
    pub fn has_swap(&self) -> bool {
        !self.swaps.is_empty()
    }
}