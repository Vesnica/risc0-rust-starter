use common::{encode_hex_u32, encode_hex_u8, Proof};
use methods::{MULTIPLY_CONTENTS, MULTIPLY_ID, MULTIPLY_PATH};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    let a: u64 = 1023;
    let b: u64 = 23;

    let mut prover = Prover::new(MULTIPLY_CONTENTS, MULTIPLY_ID).unwrap();
    prover.add_input(to_vec(&a).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&b).unwrap().as_slice()).unwrap();

    println!("Executing {}", MULTIPLY_PATH.split('/').last().unwrap());
    let receipt = prover.run().unwrap();

    let c: u64 = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
    println!("Result: {}, and I can prove it!", c);

    let proof = Proof {
        journal: encode_hex_u8(receipt.get_journal().unwrap()),
        seal: encode_hex_u32(receipt.get_seal().unwrap()),
    };
    confy::store_path("./proof.toml", proof).unwrap();
}
