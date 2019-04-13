use cryptoxide::sha3::Sha3;
use cryptoxide::sha2::Sha256;
use cryptoxide::digest::Digest;
use super::error::KeyError;
use secp256k1::{ PublicKey, Message, Signature, util, verify };

pub struct XPub(PublicKey);

impl XPub {
  pub(crate) fn new(key: PublicKey) -> Self {
    Self(key)
  }

  pub fn serialize(&self) -> Vec<u8> {
    let bytes: &[u8] = &self.0.serialize();
    Vec::from(bytes)
  }

  pub fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, KeyError> {
    if signature.len() != util::SIGNATURE_SIZE {
      return Err(KeyError::InvalidSignature(signature.len(), util::SIGNATURE_SIZE));
    }
    let mut sha3 = Sha3::keccak256();
    sha3.input(data);
    let mut out = [0u8; util::MESSAGE_SIZE];
    sha3.result(&mut out);

    let message = Message::parse(&out);

    Signature::parse_slice(signature)
      .map(|signature| verify(&message, &signature, &self.0))
      .map_err(|err| err.into() )
  }

  pub fn sha256(&self) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut out = [0u8; 32];
    hasher.input(&self.0.serialize());
    hasher.result(&mut out);
    out
  }
}
