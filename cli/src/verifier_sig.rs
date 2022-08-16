use common::{decode_hex_u32, decode_hex_u8, Message, Proof, SignMessageCommit};
use methods::SIGNATURE_ID;
use risc0_zkp::core::sha::Digest;
use risc0_zkvm::host::{Receipt, Result};
use risc0_zkvm::serde::from_slice;

pub struct SignatureWithReceipt {
    receipt: Receipt,
}

impl SignatureWithReceipt {
    pub fn get_commit(&self) -> Result<SignMessageCommit> {
        let msg = self.receipt.get_journal_vec()?;
        Ok(from_slice(msg.as_slice()).unwrap())
    }

    pub fn get_identity(&self) -> Result<Digest> {
        let commit = self.get_commit().unwrap();
        Ok(commit.identity)
    }

    pub fn get_message(&self) -> Result<Message> {
        let commit = self.get_commit().unwrap();
        Ok(commit.msg)
    }
}

fn main() {
    let proof: Proof = confy::load_path("./proof.toml").unwrap();
    let journal = decode_hex_u8(&proof.journal).unwrap();
    let seal = decode_hex_u32(&proof.seal).unwrap();
    let receipt = Receipt::new(&journal, &seal).unwrap();
    match receipt.verify(SIGNATURE_ID) {
        Ok(_) => println!("Verified OK!"),
        Err(_) => println!("Verify Failed!"),
    };
    let sign = SignatureWithReceipt { receipt };
    println!("Commit: {:?}", sign.get_commit().unwrap());
}
