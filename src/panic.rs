use core::panic::PanicInfo;

extern "C" {
    fn __cortex_a_rt_platform_halt() -> !;
}

#[panic_handler]
fn handler(info: &PanicInfo) -> ! {
    #[cfg(feature = "defmt")]
    {
        defmt::error!("=== KERNEL PANIC ===");
        if let Some(location) = info.location() {
            defmt::error!(" @ {}:{}", location.file(), location.line());
        } else {
            defmt::error!(" <location unknown>");
        }

        if let Some(message) = info.message().as_str() {
            defmt::error!("Message: {}", message);
        }
    }

    unsafe { __cortex_a_rt_platform_halt() };
}
