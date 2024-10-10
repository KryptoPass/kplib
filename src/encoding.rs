use base64::{Engine as _, engine::general_purpose};
use hex;
use crate::CryptoError;

pub struct EncodingOps;

impl EncodingOps {
    pub fn encode(encoding: &str, data: &[u8]) -> Result<String, CryptoError> {
        match encoding.to_lowercase().as_str() {
            "base64" => Ok(general_purpose::STANDARD.encode(data)),
            "hex" => Ok(hex::encode(data)),
            _ => Err(CryptoError::UnsupportedAlgorithm(encoding.to_string())),
        }
    }

    pub fn decode(encoding: &str, data: &str) -> Result<Vec<u8>, CryptoError> {
        match encoding.to_lowercase().as_str() {
            "base64" => general_purpose::STANDARD.decode(data).map_err(|e| CryptoError::EncodingError(e.to_string())),
            "hex" => hex::decode(data).map_err(|e| CryptoError::EncodingError(e.to_string())),
            _ => Err(CryptoError::UnsupportedAlgorithm(encoding.to_string())),
        }
    }
}