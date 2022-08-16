use common::{encode_hex_u32, encode_hex_u8, Message, Passphrase, Proof, SigningRequest};
use methods::{SIGNATURE_ELF, SIGNATURE_ID, SIGNATURE_PATH};
use risc0_zkvm::host::{Prover, Receipt, Result};
use risc0_zkvm::serde::to_vec;
use sha2::{Digest, Sha256};

pub fn sign(pass_str: impl AsRef<[u8]>, msg_str: impl AsRef<[u8]>) -> Result<Receipt> {
    let mut pass_hasher = Sha256::new();
    pass_hasher.update(pass_str);
    let mut pass_hash = [0u8; 32];
    pass_hash.copy_from_slice(&pass_hasher.finalize());
    println!("Password Hash: {}", Message { msg: pass_hash });

    let mut msg_hasher = Sha256::new();
    msg_hasher.update(msg_str);
    let mut msg_hash = [0u8; 32];
    msg_hash.copy_from_slice(&msg_hasher.finalize());
    println!("Message Hash: {:?}", Message { msg: msg_hash });

    let pass = Passphrase { pass: pass_hash };
    let msg = Message { msg: msg_hash };

    let params = SigningRequest {
        passphrase: pass,
        msg,
    };

    let mut prover = Prover::new(SIGNATURE_ELF, SIGNATURE_ID)?;
    let vec = to_vec(&params).unwrap();
    prover.add_input(vec.as_slice())?;
    println!("Executing {}", SIGNATURE_PATH.split('/').last().unwrap());
    let receipt = prover.run()?;
    Ok(receipt)
}

fn main() {
    let pass_str = "Aa888888";
    let msg_str = "This message was signed by me";
    let receipt = sign(pass_str, msg_str).unwrap();
    let proof = Proof {
        journal: encode_hex_u8(receipt.get_journal().unwrap()),
        seal: encode_hex_u32(receipt.get_seal().unwrap()),
    };
    confy::store_path("./proof.toml", proof).unwrap();
}
