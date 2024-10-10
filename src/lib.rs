mod error;

use semver::Version;
use std::env;

pub use error::{Error, Result};

pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn crate_version() -> Result<Version> {
    match Version::parse(CRATE_VERSION) {
        Ok(version) => Ok(version),
        Err(err) => Err(Error::InvalidPackageVersion(err)),
    }
}

use openssl::nid::Nid;
use openssl::hash::MessageDigest;
use openssl::symm::Cipher;
use openssl::rsa::Padding;

mod hash;
mod symmetric;
mod asymmetric;
mod encoding;

pub use hash::HashOps;
pub use symmetric::SymmetricOps;
pub use asymmetric::AsymmetricOps;
pub use encoding::EncodingOps;

#[derive(Debug)]
pub enum CryptoError {
    UnsupportedAlgorithm(String),
    OpenSSLError(openssl::error::ErrorStack),
    EncodingError(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CryptoError::UnsupportedAlgorithm(alg) => write!(f, "Unsupported algorithm: {}", alg),
            CryptoError::OpenSSLError(e) => write!(f, "OpenSSL error: {}", e),
            CryptoError::EncodingError(e) => write!(f, "Encoding error: {}", e),
        }
    }
}

impl std::error::Error for CryptoError {}

impl From<openssl::error::ErrorStack> for CryptoError {
    fn from(error: openssl::error::ErrorStack) -> Self {
        CryptoError::OpenSSLError(error)
    }
}

pub struct CryptoOps;

impl CryptoOps {
    pub fn get_algorithm(name: &str) -> Result<Nid, CryptoError> {
        Nid::from_string(name).map_err(|_| CryptoError::UnsupportedAlgorithm(name.to_string()))
    }

    pub fn hash(algorithm: &str, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nid = Self::get_algorithm(algorithm)?;
        let md = MessageDigest::from_nid(nid).ok_or_else(|| CryptoError::UnsupportedAlgorithm(algorithm.to_string()))?;
        HashOps::hash(md, data)
    }

    pub fn symmetric_encrypt(algorithm: &str, key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nid = Self::get_algorithm(algorithm)?;
        let cipher = Cipher::from_nid(nid).ok_or_else(|| CryptoError::UnsupportedAlgorithm(algorithm.to_string()))?;
        SymmetricOps::encrypt(cipher, key, iv, data)
    }

    pub fn symmetric_decrypt(algorithm: &str, key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nid = Self::get_algorithm(algorithm)?;
        let cipher = Cipher::from_nid(nid).ok_or_else(|| CryptoError::UnsupportedAlgorithm(algorithm.to_string()))?;
        SymmetricOps::decrypt(cipher, key, iv, data)
    }

    pub fn asymmetric_encrypt(algorithm: &str, public_key: &[u8], data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nid = Self::get_algorithm(algorithm)?;
        AsymmetricOps::encrypt(nid, public_key, data)
    }

    pub fn asymmetric_decrypt(algorithm: &str, private_key: &[u8], data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nid = Self::get_algorithm(algorithm)?;
        AsymmetricOps::decrypt(nid, private_key, data)
    }

    pub fn encode(encoding: &str, data: &[u8]) -> Result<String, CryptoError> {
        EncodingOps::encode(encoding, data)
    }

    pub fn decode(encoding: &str, data: &str) -> Result<Vec<u8>, CryptoError> {
        EncodingOps::decode(encoding, data)
    }
}