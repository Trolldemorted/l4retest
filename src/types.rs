use core::clone::Clone;
use core::marker::Copy;
use core::prelude::rust_2024::derive;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct L4CapIndex(pub usize);
