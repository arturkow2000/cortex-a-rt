use core::{fmt::Arguments, panic::PanicInfo};

extern "C" {
    fn __cortex_a_rt_platform_halt() -> !;
}

#[cfg(feature = "unwind")]
pub fn do_backtrace() {
    use core::ffi::c_void;

    use unwinding::abi::{
        UnwindContext, UnwindReasonCode, _Unwind_Backtrace, _Unwind_GetIP, _Unwind_GetRegionStart,
    };

    struct CallbackData {
        counter: usize,
    }
    extern "C" fn callback(
        unwind_ctx: &mut UnwindContext<'_>,
        arg: *mut c_void,
    ) -> UnwindReasonCode {
        let data = unsafe { &mut *(arg as *mut CallbackData) };
        data.counter += 1;

        let pc = _Unwind_GetIP(unwind_ctx);
        let entry = _Unwind_GetRegionStart(unwind_ctx);
        let off = pc - entry;

        defmt::error!("{}:{:#08x} - {:#08x}+{:#x}", data.counter, pc, entry, off);
        UnwindReasonCode::NO_REASON
    }

    let mut data = CallbackData { counter: 0 };
    _Unwind_Backtrace(callback, &mut data as *mut _ as _);
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

    //#[cfg(feature = "unwind")]
    //{
    //    do_backtrace();
    //}

    unsafe { __cortex_a_rt_platform_halt() };
}
