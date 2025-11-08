#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(clippy::missing_safety_doc)]
pub mod elf;
pub mod env;
pub mod types;
pub mod utcb;
use crate::{
    elf::{AT_L4_ENV, AT_NULL, Elf64AuxV},
    env::L4ReEnvPtr,
    utcb::UtcbPtr,
};
#[cfg(not(test))]
use core::panic::PanicInfo;
use core::{
    arch::{asm, global_asm},
    sync::atomic::Ordering,
};

const L4_SYSF_SEND: usize = 1;
const L4_IPC_NEVER: usize = 0;
const L4_PROTO_LOG: isize = -13;
const L4_MSGTAG_SCHEDULE: usize = 0x2000;
const L4_VCON_WRITE_OP: usize = 0;

global_asm!("
.global _start
_start:
    xor ebp, ebp
    pop rdi
    mov rsi, rsp
    and rsp,  ~15
    call {main}
",
main = sym main);

// l4re-core/libc/uclibc-ng/contrib/uclibc/libc/misc/internals/uClibc_main.c:__uClibc_main
unsafe fn main(argc: usize, argv: *const *const u8) {
    unsafe {
        // The envrionment begins right after argv
        let mut __environ = argv.add(argc + 1);

        // If the first thing after argc is the arguments
        // then the environment is empty.
        if __environ as *const u8 == *argv {
            __environ = argv.add(argc)
        }

        let mut aux_dat: *const u64 = __environ as _;
        while *aux_dat != 0 {
            aux_dat = aux_dat.add(1);
        }
        aux_dat = aux_dat.add(1);

        aux_init(aux_dat as _);
        l4_vcon_send_u();
    };

    loop {}
}

unsafe fn aux_init(mut av: *const Elf64AuxV) {
    unsafe {
        while (*av).a_type != AT_NULL {
            if (*av).a_type == AT_L4_ENV {
                env::L4RE_GLOBAL_ENV_ADDRESS.store((*av).a_val, Ordering::SeqCst);
            }
            av = av.add(1);
        }
    }
}

// force_write_usize(0x14, 0x42);
pub unsafe fn force_write_usize(fi_address: usize, value: usize) {
    unsafe {
        let dst: *mut usize = fi_address as _;
        *dst = value;
    }
}

// l4re-core/l4sys/include/vcon.h
unsafe fn l4_vcon_send_u() {
    let utcb = UtcbPtr::new();
    let env = L4ReEnvPtr::new();
    let mut mr = utcb.get_message_registers();
    unsafe {
        mr.write(0, L4_VCON_WRITE_OP);
        mr.write(1, 8);
        mr.write(2, 0x0a41414141414141);
        l4_ipc_send(
            env.get_log().0,
            utcb.address,
            l4_msgtag(L4_PROTO_LOG, 2 + 1, 0, L4_MSGTAG_SCHEDULE),
            L4_IPC_NEVER,
        );
    }
}

// l4re-core/l4sys/include/ipc.h
unsafe fn l4_ipc_send(dest: usize, utcb: usize, tag: usize, timeout: usize) {
    unsafe { l4_ipc(dest, utcb, L4_SYSF_SEND, 0, tag, 0, timeout) }
}

// l4re-core/l4sys/include/ARCH-amd64/L4API-l4f/ipc.h
unsafe fn l4_ipc(
    dest: usize,
    _utcb: usize,
    flags: usize,
    slabel: usize,
    tag: usize,
    _rlabel: usize,
    timeout: usize,
) {
    unsafe {
        asm!(
            "syscall",
            in("rsi") slabel,
            in("r8") timeout,
            in("rax") tag,
            in("rdx") dest | flags,
        );
    }
}

// l4re-core/l4sys/include/types.h
fn l4_msgtag(label: isize, words: usize, items: usize, flags: usize) -> usize {
    (label as usize) << 16 | words & 0x3f | (items & 0x3f) << 6 | flags & 0xf000
}

#[cfg(not(test))]
#[cfg_attr(not(test), panic_handler)]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(3, 3);
    }
}
