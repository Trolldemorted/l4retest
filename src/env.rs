use core::{
    ffi::c_void,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::types::L4CapIndex;

pub static L4RE_GLOBAL_ENV_ADDRESS: AtomicUsize = AtomicUsize::new(0);

pub struct L4ReEnvPtr {
    pub address: usize,
}

#[repr(C)]
pub struct L4ReEnv {
    /// Parent object-capability
    parent: L4CapIndex,
    /// Region map object-capability
    rm: L4CapIndex,
    /// Memory allocator object-capability
    mem_alloc: L4CapIndex,
    /// Logging object-capability
    log: L4CapIndex,
    /// Object-capability of the first user thread
    main_thread: L4CapIndex,
    /// Object-capability of the factory available to the task
    factory: L4CapIndex,
    /// Object capability for the scheduler set to use
    scheduler: L4CapIndex,
    /// ITAS services object-capability
    itas: L4CapIndex,
    /// Object-capability of the debug events service
    dbg_events: L4CapIndex,
    /// First capability index available to the application
    first_free_cap: L4CapIndex,
    /// UTCB area of the task
    utcb_area: usize, //TODO size?
    /// First UTCB within the UTCB area available to the application
    first_free_utcb: usize, //TODO size?
    /// Pointer to the first entry in the initial objects array which contains
    /// #l4re_env_cap_entry_t elements. The array is terminated by an invalid
    /// entry with a `flags` value of `~0ul`.
    caps: *const c_void,
}

impl L4ReEnvPtr {
    pub fn new() -> Self {
        L4ReEnvPtr {
            address: L4RE_GLOBAL_ENV_ADDRESS.load(Ordering::SeqCst),
        }
    }

    pub fn get_log(&self) -> L4CapIndex {
        let ptr: *const L4ReEnv = self.address as _;
        unsafe { (*ptr).log }
    }
}
