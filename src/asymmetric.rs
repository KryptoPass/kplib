use openssl::rsa::{Rsa, Padding};
use openssl::nid::Nid;
use crate::CryptoError;

pub struct AsymmetricOps;

impl AsymmetricOps {
    pub fn encrypt(nid: Nid, public_key: &[u8], data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        match nid {
            Nid::RSA => {
                let rsa = Rsa::public_key_from_pem(public_key)?;
                let mut buf = vec![0; rsa.size() as usize];
                let encrypted_size = rsa.public_encrypt(data, &mut buf, Padding::PKCS1)?;
                buf.truncate(encrypted_size);
                Ok(buf)
            },
            _ => Err(CryptoError::UnsupportedAlgorithm(format!("{:?}", nid))),
        }
    }

    pub fn decrypt(nid: Nid, private_key: &[u8], data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        match nid {
            Nid::RSA => {
                let rsa = Rsa::private_key_from_pem(private_key)?;
                let mut buf = vec![0; rsa.size() as usize];
                let decrypted_size = rsa.private_decrypt(data, &mut buf, Padding::PKCS1)?;
                buf.truncate(decrypted_size);
                Ok(buf)
            },
            _ => Err(CryptoError::UnsupportedAlgorithm(format!("{:?}", nid))),
        }
    }
}