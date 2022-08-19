use common::{encode_hex_u32, encode_hex_u8, PasswordRequest, Proof};
use methods::{PASSWORD_CONTENTS, PASSWORD_ID, PASSWORD_PATH};
use rand::{thread_rng, RngCore};
use risc0_zkp::core::sha::Digest;
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    let mut rng = thread_rng();
    let mut salt = [0u8; 32];
    rng.fill_bytes(&mut salt);

    let request = PasswordRequest {
        password: "S00perSecr1t!!!".into(),
        salt,
    };

    let mut prover = Prover::new(PASSWORD_CONTENTS, PASSWORD_ID).unwrap();

    // Adding input to the prover makes it readable by the guest
    let vec = to_vec(&request).unwrap();
    prover.add_input(&vec).unwrap();

    println!("Executing {}", PASSWORD_PATH.split('/').last().unwrap());
    let receipt = prover.run().unwrap();
    let password_hash: Digest = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
    println!("Password hash is: {}", &password_hash);

    let proof = Proof {
        journal: encode_hex_u8(receipt.get_journal().unwrap()),
        seal: encode_hex_u32(receipt.get_seal().unwrap()),
    };
    confy::store_path("./proof.toml", proof).unwrap();
}
