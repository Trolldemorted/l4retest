/// SAFETY: Respects the 64-bit System-V ABI.
#[unsafe(naked)]
pub extern "sysv64" fn mmap(a: u64, b: u64) -> u64 {
    core::arch::naked_asm!(
        "mov edx, ebx",     // Save ebx
        "mov eax, 90",      // eax = syscall number
        "lea ebx, [esp+4]", // arg1 = pointer to arguments
        "int 0x80",
        "mov ebx, edx", // Restore ebx
        "ret"
    );
}
