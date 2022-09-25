#[repr(C)]
#[non_exhaustive]
pub struct ExceptionFrame {
    pub lr: u32,
    pub sp: u32,
    pub spsr: u32,
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub pc: u32,
}

#[cfg(feature = "defmt")]
fn dump_exception_frame(frame: &ExceptionFrame) {
    use defmt::error;
    error!("Undefined instruction at 0x{:08x}", frame.pc);
    error!(
        "r0  0x{:08x}  r1  0x{:08x}  r2  0x{:08x}  r3  0x{:08x}",
        frame.r0, frame.r1, frame.r2, frame.r3
    );
    error!(
        "r4  0x{:08x}  r5  0x{:08x}  r6  0x{:08x}  r7  0x{:08x}",
        frame.r4, frame.r5, frame.r6, frame.r7
    );
    error!(
        "r8  0x{:08x}  r9  0x{:08x}  r10 0x{:08x}  r11 0x{:08x}",
        frame.r8, frame.r9, frame.r10, frame.r11
    );
    error!(
        "r12 0x{:08x}  sp  0x{:08x}  lr  0x{:08x}  pc  0x{:08x}",
        frame.r12, frame.sp, frame.lr, frame.pc
    );
}

#[no_mangle]
unsafe extern "C" fn __cortex_a_undefined_handler(frame: &mut ExceptionFrame) {
    #[cfg(feature = "defmt")]
    {
        dump_exception_frame(frame);
    }

    crate::util::panic!("Undefined instruction at 0x{:08x}", frame.pc);
}

#[no_mangle]
unsafe extern "C" fn __cortex_a_prefetch_abort(frame: &mut ExceptionFrame) {
    #[cfg(feature = "defmt")]
    {
        dump_exception_frame(frame);
    }

    crate::util::panic!("Prefetch abort at 0x{:08x}", frame.pc);
}

#[no_mangle]
unsafe extern "C" fn __cortex_a_data_abort(frame: &mut ExceptionFrame) {
    #[cfg(feature = "defmt")]
    {
        dump_exception_frame(frame);
    }

    crate::util::panic!("Data abort at 0x{:08x}", frame.pc);
}
