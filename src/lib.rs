#![no_std]

extern crate alloc;

use core::time::Duration;

mod constants;
mod regs;
mod err;
mod mci;

pub use mci::*;

pub trait Kernel {
    fn sleep(duration: Duration);
}

pub(crate) fn sleep(duration: Duration) {
    extern "Rust" {
        fn _phytium_mci_sleep(duration: Duration);
    }

    unsafe {
        _phytium_mci_sleep(duration);
    }
}

#[macro_export]
macro_rules! set_impl {
    ($t: ty) => {
        #[no_mangle]
        unsafe fn _phytium_mci_sleep(duration: core::time::Duration) {
            <$t as $crate::Kernel>::sleep(duration)
        }
    };
}
