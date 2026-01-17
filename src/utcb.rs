use core::arch::asm;
pub use core::prelude::*;
use core::ptr;

const L4_UTCB_MSG_REGS_OFFSET: usize = 0;
const _L4_UTCB_GENERIC_DATA_SIZE: usize = 63;

pub struct UtcbPtr {
    pub address: usize,
}

impl UtcbPtr {
    pub unsafe fn new() -> Self {
        unsafe {
            let mut address;
            asm!(
                "mov {address}, gs:0",
                address = out(reg) address,
            );

            UtcbPtr { address }
        }
    }

    /// l4re-core/l4sys/include/utcb.h
    pub fn get_message_registers(&self) -> L4MsgRegsPtr {
        L4MsgRegsPtr::new(self.address + L4_UTCB_MSG_REGS_OFFSET)
    }
}

pub struct L4MsgRegsPtr {
    pub address: usize,
}

impl L4MsgRegsPtr {
    pub fn new(address: usize) -> Self {
        Self { address }
    }

    pub unsafe fn write(&mut self, index: usize, value: usize) {
        let ptr: *mut usize = (self.address + 8 * index) as _;
        unsafe { *ptr = value };
    }

    /// TODO: Build safe abstraction
    pub unsafe fn memcopy(&mut self, index: usize, value: &[u8]) {
        let ptr: *mut u8 = (self.address + 8 * index) as _;
        unsafe {
            ptr::copy(value.as_ptr(), ptr, value.len());
        }
    }
}
