pub const AT_NULL: usize = 0;
pub const AT_L4_AUX: usize = 0xf0;
pub const AT_L4_ENV: usize = 0xf1;

pub struct ElfAT(pub usize);

#[repr(C)]
pub struct Elf64AuxV {
    pub a_type: usize,
    pub a_val: usize,
}
