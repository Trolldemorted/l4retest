use core::arch::asm;
pub use core::prelude::*;

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
}
