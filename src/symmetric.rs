use openssl::symm::{Cipher, Crypter, Mode};
use crate::CryptoError;

pub struct SymmetricOps;

impl SymmetricOps {
    pub fn encrypt(cipher: Cipher, key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, iv)?;
        let mut ciphertext = vec![0; data.len() + cipher.block_size()];
        let count = crypter.update(data, &mut ciphertext)?;
        let rest = crypter.finalize(&mut ciphertext[count..])?;
        ciphertext.truncate(count + rest);
        Ok(ciphertext)
    }

    pub fn decrypt(cipher: Cipher, key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, iv)?;
        let mut plaintext = vec![0; data.len() + cipher.block_size()];
        let count = crypter.update(data, &mut plaintext)?;
        let rest = crypter.finalize(&mut plaintext[count..])?;
        plaintext.truncate(count + rest);
        Ok(plaintext)
    }
}