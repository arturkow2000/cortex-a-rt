#![cfg_attr(not(test), no_std)]

#[cfg(feature = "defmt")]
#[allow(unused_imports)]
#[macro_use(error)]
extern crate defmt;

#[cfg(feature = "log")]
#[allow(unused_imports)]
#[macro_use(error)]
extern crate log;

#[cfg(all(feature = "defmt", feature = "log"))]
compile_error!("enable either \"defmt\" or \"log\" but not both");

extern crate cortex_a_rt_macros as macros;

#[allow(unused_imports)]
use core::arch::global_asm;

pub use macros::*;

#[cfg(armv7a)]
global_asm! {
    r#"
    .section .init0, "ax"
    .global __cortex_a_rt_reset
    __cortex_a_rt_reset:
    b __cortex_a_init
    b __cortex_a_excp_undefined
    b __cortex_a_excp_syscall
    b __cortex_a_excp_prefetch_abort
    b __cortex_a_excp_data_abort
    b __cortex_a_excp_reserved
    b __cortex_a_excp_irq
    b __cortex_a_excp_fiq
    "#
}

// Dummy exception handlers
#[cfg(armv7a)]
global_asm! {
    r#"
    .section .except, "ax"
    # This macro saves all registers and calls exception handler. Upon exit it
    # restores context to resume execution.
    # Should not be called from FIQ mode as it assumes r8-r12 are not banked.
    .macro excp_handle, mode, correction, handler
        .if \correction
        sub lr, #\correction
        .endif

        # Save all non-banked registers, LR is the address where exception
        # occurred (was PC before exception entry)
        push {{r0-r12, lr}}
        mrs r0, spsr
        push {{r0}}

        # FIXME: assumes exception occurred in SVC mode and will give wrong register
        # readings when that is not the case.
        cps #0x13
        mov r0, lr
        mov r1, sp
        cps #\mode
        push {{r0, r1}}

        mov r0, sp
        tst sp, #4
        subeq sp, #4 @ align stack
        push {{r0}}

        bl \handler

        # Restore context, handler must either modify frame to ensure execution
        # can resume correctly or never return.

        pop {{r0}}
        mov sp, r0
        ldmfd sp!, {{r0-r12, pc}}^
    .endm

    __cortex_a_excp_undefined:
        excp_handle 0x1b, 0, __cortex_a_undefined_handler
    __cortex_a_excp_syscall:
        b .
    __cortex_a_excp_prefetch_abort:
        excp_handle 0x17, 4, __cortex_a_prefetch_abort
    __cortex_a_excp_data_abort:
        excp_handle 0x17, 4, __cortex_a_data_abort
    __cortex_a_excp_reserved:
        b .
    __cortex_a_excp_irq:
        sub lr, #4
        push {{r0-r12, lr}}

        mov r0, sp
        tst sp, #4
        subeq sp, #4
        push {{r0}}

        bl __cortex_a_irq_handler

        pop {{r0}}
        mov sp, r0
        ldmfd sp!, {{r0-r12, pc}}^
    __cortex_a_excp_fiq:
        b .

    .section .weak_default, "ax"
    .weak __cortex_a_irq_handler
    .type __cortex_a_irq_handler, #function
    __cortex_a_irq_handler:
        bx lr
    .size __cortex_a_irq_handler, . - __cortex_a_irq_handler
    "#
}

#[cfg(armv7a)]
global_asm! {
    r#"
    .section .init1, "ax"
    __cortex_a_init:
        # TODO: need to handle secure monitor and HYP modes
        cpsid if, #0x13
        ldr r0, =__cortex_a_rt_reset
        mcr p15, 0, r0, c12, c0, 0 @ set VBAR
        mrc p15, 0, r0, c1, c0, 0 @ read SCTRL
        bic r0, #0x7 @ turn off MMU, D-cache and alignment check
        bic r0, #0x40000000 @ set exception to run in ARM mode
        bic r0, #0x3000 @ disable I-cache and set vectors to VBAR
        mcr p15, 0, r0, c1, c0, 0 @ write SCTRL

        # Setup stack for supervisor mode
        ldr sp, =__stack_end

        # Setup stack for other modes
        ldr r0, =__ab_stack_end
        cpsid if, #0x12 @ irq
        mov sp, r0

        cpsid if, #0x11 @ fiq
        mov sp, r0

        cpsid if, #0x17 @ abort
        mov sp, r0

        cpsid if, #0x1b @ undefined
        mov sp, r0

        cpsid if, #0x1f @ system
        mov sp, r0

        cpsid if, #0x13 @ continue execution in supervisor mode

        # Zero out BSS
        ldr r0, =__sbss
        ldr r1, =__ebss
        subs r1, r1, r0 @ R1 = BSS size in bytes
        beq 2f
        ldr r2, =0
        1:
        str r2, [r0], #4
        subs r1, r1, #4
        bne 1b
        2:
        bl main
        udf #0
    "#
}

#[cfg(feature = "single-core-critical-section")]
mod critical_section;

pub mod exceptions;
mod halt;
pub mod interrupt;
#[cfg(feature = "panic_handler")]
mod panic;
mod util;
