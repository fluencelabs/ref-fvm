use randomx_rs::RandomXFlag;
use randomx_rs::RandomXCache;
use randomx_rs::RandomXVM;

use super::Context;
use crate::kernel::Kernel;
use crate::kernel::Result;

pub const RANDOMX_HASH_SIZE: usize = 32;

pub fn run_randomx(
    context: Context<'_, impl Kernel>,
    k_addr: u32,
    k_len: u32,
    h_addr: u32,
    h_len: u32
) -> Result<[u8; RANDOMX_HASH_SIZE]> {
    let k = context.memory.try_slice(k_addr, k_len)?;
    let h = context.memory.try_slice(h_addr, h_len)?;
    println!("run_randomx: host syscall start with {k:?} {h:?}");

    let randomx_flags = RandomXFlag::get_recommended_flags();
    let cache = RandomXCache::new(randomx_flags, k).unwrap();
    let vm = RandomXVM::new(randomx_flags, Some(cache), None).unwrap();
    let hash = vm.calculate_hash(h).unwrap();
    let mut result = [0u8; RANDOMX_HASH_SIZE];

    for i in 0..32 {
        result[i] = hash[i];
    }

    Ok(result)
}
