#![no_main]
#![no_std]

use common::{SignMessageCommit, SigningRequest};
use risc0_zkvm_guest::{env, sha};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let request: SigningRequest = env::read();
    env::commit(&SignMessageCommit {
        identity: *sha::digest_u8_slice(&request.passphrase.pass),
        msg: request.msg,
    });
}
