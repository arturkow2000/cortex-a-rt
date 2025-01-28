use core::panic::PanicInfo;

extern "C" {
    fn __cortex_a_rt_platform_halt() -> !;
}

#[panic_handler]
fn handler(info: &PanicInfo) -> ! {
    #[cfg(any(feature = "defmt", feature = "log"))]
    {
        error!("=== KERNEL PANIC ===");
        if let Some(location) = info.location() {
            error!(" @ {}:{}", location.file(), location.line());
        } else {
            error!(" <location unknown>");
        }

        if let Some(message) = info.message().as_str() {
            error!("Message: {}", message);
        }
    }
    #[cfg(not(any(feature = "defmt", feature = "log")))]
    {
        let _ = info;
    }

    unsafe { __cortex_a_rt_platform_halt() };
}
