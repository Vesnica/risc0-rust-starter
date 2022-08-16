use common::{decode_hex_u32, decode_hex_u8, Proof};
use methods::PASSWORD_ID;
use risc0_zkvm::host::Receipt;

fn main() {
    let proof: Proof = confy::load_path("./proof.toml").unwrap();
    let journal = decode_hex_u8(&proof.journal).unwrap();
    let seal = decode_hex_u32(&proof.seal).unwrap();
    let receipt = Receipt::new(&journal, &seal).unwrap();
    match receipt.verify(PASSWORD_ID) {
        Ok(_) => println!("Verified OK!"),
        Err(_) => println!("Verify Failed!"),
    };
}
