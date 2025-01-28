#[no_mangle]
extern "C" fn __cortex_a_rt_platform_halt_default() -> ! {
    loop {
        unsafe {
            #[cfg(armv7a)]
            core::arch::asm!("wfi", options(nomem, nostack));
        };
    }
}
