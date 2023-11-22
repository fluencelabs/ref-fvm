use fvm_shared::error::ExitCode;
use fvm_shared::sys::out::vm::MessageContext;
use fvm_shared::sys::SyscallSafe;

use super::error::Abort;
use super::Context;
use crate::kernel::Kernel;

pub fn run_randomx(context: Context<'_, impl Kernel>, k: u32, h: u32) -> Result<(), Abort> {
    let result = context.kernel.run_randomx(k, h).unwrap();
    Ok(())
}