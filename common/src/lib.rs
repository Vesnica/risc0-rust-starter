use core::fmt::{Debug, Display, Formatter, Write};
use core::num::ParseIntError;
use risc0_zkp::core::sha::Digest;
use serde::{Deserialize, Serialize};

pub fn decode_hex_u8(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn decode_hex_u32(s: &str) -> Result<Vec<u32>, ParseIntError> {
    (0..s.len())
        .step_by(8)
        .map(|i| u32::from_str_radix(&s[i..i + 8], 16))
        .collect()
}

pub fn encode_hex_u8(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn encode_hex_u32(bytes: &[u32]) -> String {
    let mut s = String::with_capacity(bytes.len() * 8);
    for &b in bytes {
        write!(&mut s, "{:08x}", b).unwrap();
    }
    s
}

#[derive(Serialize, Deserialize)]
pub struct Proof {
    pub journal: String,
    pub seal: String,
}

impl ::std::default::Default for Proof {
    fn default() -> Self {
        Self {
            journal: Default::default(),
            seal: Default::default(),
        }
    }
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Message {
    pub msg: [u8; 32],
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        for word in self.msg {
            core::write!(f, "{:02x?}", word)?;
        }
        Ok(())
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        for word in self.msg {
            core::write!(f, "{:02x?}", word)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Passphrase {
    pub pass: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SigningRequest {
    pub passphrase: Passphrase,
    pub msg: Message,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SignMessageCommit {
    pub identity: Digest,
    pub msg: Message,
}
