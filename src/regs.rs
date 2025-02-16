#![allow(unused)] 

use core::{marker::PhantomData, ptr::NonNull, time::Duration};
use bitflags::{bitflags, Flags};
use log::info;
use crate::sleep;

/*
 * Create a contiguous bitmask starting at bit position @l and ending at
 * position @h. For example
 * GENMASK_ULL(39, 21) gives us the 64bit vector 0x000000ffffe00000.
 */
#[macro_export]
macro_rules! genmask {
    ($h:expr, $l:expr) => {
        (((!0u32) - (1u32 << $l) + 1) & ((!0u32) >> (32 - 1 - $h)))
    };
}

#[macro_export]
macro_rules! genmask_ull {
    ($h:expr, $l:expr) => {
        (((!0u64) - (1u64 << $l) + 1) & ((!0u64) >> (64 - 1 - $h)))
    };
}

/* set 32-bit register [a:b] as x, where a is high bit, b is low bit, x is setting/getting value */
#[macro_export]
macro_rules! get_reg32_bits {
    ($reg:expr, $a:expr, $b:expr) => {
        ($reg & genmask!($a, $b)) >> $b
    };
}

#[macro_export]
macro_rules! set_reg32_bits {
    ($reg:expr, $a:expr, $b:expr) => {
        (($reg << $b) & genmask!($a, $b))
    };
}

pub struct Reg<E:RegError> {
    pub addr: NonNull<u8>,
    _marker: PhantomData<E>
}

impl<E:RegError> Reg<E> {
    pub fn new(addr: NonNull<u8>) -> Self {
        Self { 
            addr,
            _marker: PhantomData
        }
    }

    pub fn read_32(&self, reg: u32) -> u32 {
        unsafe {
            let ptr = self.addr.add(reg as _);
            ptr.cast().read_volatile()
        }
    }

    pub fn write_32(&self, reg: u32, val: u32) {
        unsafe {
            let ptr = self.addr.add(reg as _);
            ptr.cast().write_volatile(val);
        }
    }

    pub fn read_reg<F: FlagReg>(&self) -> F {
        F::from_bits_retain(self.read_32(F::REG))
    }

    pub fn write_reg<F: FlagReg>(&self, val: F) {
        self.write_32(F::REG, val.bits())
    }

    pub fn modify_reg<F: FlagReg>(&self, f: impl Fn(F) -> F) {
        let old = self.read_reg::<F>();
        self.write_reg(f(old));
    }

    pub fn wait_for<R: FlagReg, F: Fn(R) -> bool>(
        &self,
        f: F,
        interval: Duration,
        try_count: Option<usize>,
    ) -> Result<(), E> {
        for _ in 0..try_count.unwrap_or(usize::MAX) {
            if f(self.read_reg::<R>()) {
                return Ok(());
            }

            sleep(interval);
        }
        Err(E::timeout())
    }
    
}

pub trait RegError {
    fn timeout() -> Self;
}

pub trait FlagReg: Flags<Bits = u32> {
    const REG: u32;
}

pub trait OffSetReg {

}
