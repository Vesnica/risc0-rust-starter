#![no_main]
#![no_std]

use risc0_zkvm_guest::env;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let a: u64 = env::read();
    let b: u64 = env::read();
    let m: u64 = env::read();

    let result = a.checked_add(b).expect("Integer overflow") % m;
    env::commit(&result);
}
