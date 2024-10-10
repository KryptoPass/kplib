use openssl::hash::{Hasher, MessageDigest};
use crate::CryptoError;

pub struct HashOps;

impl HashOps {
    pub fn hash(md: MessageDigest, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut hasher = Hasher::new(md)?;
        hasher.update(data)?;
        Ok(hasher.finish()?.to_vec())
    }
}