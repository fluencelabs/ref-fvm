//! Syscalls for interacting with RandomX.

pub const RANDOMX_K_MAX_SIZE: usize = 60;
pub const RANDOMX_HASH_SIZE: usize = 32;

super::fvm_syscalls! {
    module = "fluence";

    pub fn run_randomx(
        k_addr: *const u8,
        k_len: u32,
        h_addr: *const u8,
        h_len: u32,
    ) -> Result<[u8; RANDOMX_HASH_SIZE]>;
}
