use common::{encode_hex_u32, encode_hex_u8, Proof};
use methods::{ADD_MOD_ELF, ADD_MOD_ID, ADD_MOD_PATH};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    let a: u64 = 1023;
    let b: u64 = 2;
    let m: u64 = 1024;

    let mut prover = Prover::new(ADD_MOD_ELF, ADD_MOD_ID).unwrap();
    prover.add_input(to_vec(&a).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&b).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&m).unwrap().as_slice()).unwrap();

    println!("Executing {}", ADD_MOD_PATH.split('/').last().unwrap());
    let receipt = prover.run().unwrap();

    let c: u64 = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
    println!("(a + b) % m = {}, and I can prove it!", c);

    let proof = Proof {
        journal: encode_hex_u8(receipt.get_journal().unwrap()),
        seal: encode_hex_u32(receipt.get_seal().unwrap()),
    };
    confy::store_path("./proof.toml", proof).unwrap();
}
