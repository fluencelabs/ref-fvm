// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use crate::SyscallResult;
use crate::sys;

pub use crate::sys::fluence::RANDOMX_HASH_SIZE;
pub use crate::sys::fluence::RANDOMX_K_MAX_SIZE;

pub fn run_randomx(k: Vec<u8>, h: Vec<u8>) -> SyscallResult<[u8; RANDOMX_HASH_SIZE]> {
    unsafe {
        sys::fluence::run_randomx(k.as_ptr(), h.len() as u32, h.as_ptr(), h.len() as u32)
    }
}
