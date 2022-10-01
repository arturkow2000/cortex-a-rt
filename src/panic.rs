use core::{fmt::Arguments, panic::PanicInfo};

extern "C" {
    fn __cortex_a_rt_platform_halt() -> !;
}

#[panic_handler]
fn handler(info: &PanicInfo) -> ! {
    #[cfg(feature = "defmt")]
    {
        defmt::error!("=== KERNEL PANIC ===");
        if let Some(location) = info.location() {
            defmt::error!(" @ {}:{}", location.file(), location.column());
        } else {
            defmt::error!(" <location unknown>");
        }

        if let Some(args) = info.message() {
            struct Formatter<'a>(&'a Arguments<'a>);
            impl defmt::Format for Formatter<'_> {
                fn format(&self, fmt: defmt::Formatter) {
                    struct FormatterInner<'a>(defmt::Formatter<'a>);
                    impl core::fmt::Write for FormatterInner<'_> {
                        fn write_str(&mut self, s: &str) -> core::fmt::Result {
                            defmt::write!(self.0, "{}", s);
                            Ok(())
                        }
                    }
                    let _ = core::fmt::write(&mut FormatterInner(fmt), *self.0);
                }
            }
            defmt::error!("Message: {}", Formatter(args));
        }
    }

    unsafe { __cortex_a_rt_platform_halt() };
}
