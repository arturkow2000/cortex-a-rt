use crate::interrupt;
use critical_section::{set_impl, Impl, RawRestoreState};

struct SingleCoreCriticalSection;
set_impl!(SingleCoreCriticalSection);

unsafe impl Impl for SingleCoreCriticalSection {
    #[allow(unreachable_code, unused_variables)]
    unsafe fn acquire() -> RawRestoreState {
        let state: u8 = {
            #[cfg(armv7a)]
            {
                let flags: u32;
                core::arch::asm!("mrs {}, cpsr", out(reg) flags, options(nostack, nomem));
                (((flags >> 5) & 6) | 1) as u8
            }
            #[cfg(not(armv7a))]
            {
                unimplemented!()
            }
        };
        interrupt::disable();
        state
    }

    unsafe fn release(state: RawRestoreState) {
        assert!(state & 1 != 0, "critical section: invalid restore state");
        #[cfg(armv7a)]
        {
            if state & 2 != 0 {
                interrupt::enable_irq();
            }

            if state & 4 != 0 {
                interrupt::enable_fiq();
            }
        }
        #[cfg(not(armv7a))]
        {
            unimplemented!()
        }
    }
}
