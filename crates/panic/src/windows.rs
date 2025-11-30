use windows_sys::Win32::System::Threading::ExitProcess;
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    unsafe {
        ExitProcess(1)
    }
}