#[allow(unused_imports)]
use core::arch::asm;

/// Enable IRQs and FIQs on calling CPU.
///
/// # SAFETY
/// Must not be called in critical section. Please use critical section API for
/// safe abstraction.
#[inline]
pub unsafe fn enable() {
    #[cfg(armv7a)]
    asm!("cpsie if", options(nostack, nomem));
    #[cfg(not(armv7a))]
    unimplemented!();
}

/// Disable IRQs and FIQs on calling CPU.
///
/// # SAFETY
/// Should be paired with later call to enable() or may result in deadlocks.
/// Please use critical section API for safe abstraction.
#[inline]
pub unsafe fn disable() {
    #[cfg(armv7a)]
    asm!("cpsid if", options(nostack, nomem));
    #[cfg(not(armv7a))]
    unimplemented!();
}

/// Enable IRQs on calling CPU (FIQs are left untouched).
///
/// # SAFETY
/// Must not be called in critical section. Please use critical section API for
/// safe abstraction.
#[inline]
pub unsafe fn enable_irq() {
    #[cfg(armv7a)]
    asm!("cpsie i", options(nostack, nomem));
    #[cfg(not(armv7a))]
    unimplemented!();
}

/// Enable FIQs on calling CPU (IRQs are left untouched).
///
/// # SAFETY
/// Must not be called in critical section. Please use critical section API for
/// safe abstraction.
#[inline]
pub unsafe fn enable_fiq() {
    #[cfg(armv7a)]
    asm!("cpsie f", options(nostack, nomem));
    #[cfg(not(armv7a))]
    unimplemented!();
}

/// Checks whether IRQs are enabled on calling CPU.
#[inline]
pub fn is_irq_enabled() -> bool {
    #[cfg(armv7a)]
    {
        let mut flags: u32;
        unsafe { asm!("mrs {}, cpsr", out(reg) flags, options(nostack, nomem)) };
        flags & (1 << 7) == 0
    }
    #[cfg(not(armv7a))]
    unimplemented!()
}

/// Checks whether FIQs are enabled on calling CPU.
pub fn is_fiq_enabled() -> bool {
    #[cfg(armv7a)]
    {
        let mut flags: u32;
        unsafe { asm!("mrs {}, cpsr", out(reg) flags, options(nostack, nomem)) };
        flags & (1 << 6) == 0
    }
    #[cfg(not(armv7a))]
    unimplemented!()
}
