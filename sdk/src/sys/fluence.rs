//! Syscalls for interacting with RandomX.

super::fvm_syscalls! {
    module = "fluence";

    pub fn run_randomx(k: u32, h: u32) -> Result<bool>;
}
