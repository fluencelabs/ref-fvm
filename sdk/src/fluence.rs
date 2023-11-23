// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use crate::SyscallResult;
use crate::sys;


pub fn run_randomx(k: u32, h: u32) -> SyscallResult<bool> {
    unsafe {
        sys::fluence::run_randomx(k, h)
    }
}
