use core::panic::PanicInfo;

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

    //#[cfg(feature = "unwind")]
    //{
    //    do_backtrace();
    //}

    unsafe { __cortex_a_rt_platform_halt() };
}
