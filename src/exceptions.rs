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

#[no_mangle]
unsafe extern "C" fn __cortex_a_undefined_handler(frame: &mut ExceptionFrame) {
    // TODO: dump more info
    panic!("Undefined instruction @ 0x{:08x}", frame.pc);
}

#[no_mangle]
unsafe extern "C" fn __cortex_a_prefetch_abort(frame: &mut ExceptionFrame) {
    // TODO: dump more info
    panic!("Prefetch abort @ 0x{:08x}", frame.pc);
}

#[no_mangle]
unsafe extern "C" fn __cortex_a_data_abort(frame: &mut ExceptionFrame) {
    // TODO: dump more info
    panic!("Data abort @ 0x{:08x}", frame.pc);
}
