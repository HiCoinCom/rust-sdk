//! Crypto provider module for RSA encryption/decryption
//!
//! This module provides the cryptographic operations needed for
//! communicating with the ChainUp API, matching the Python SDK's
//! crypto_provider module.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use md5::Md5;
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::signature::Verifier;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use rsa::{BigUint, Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};

use crate::error::{CryptoError, Result};

/// Trait for crypto provider implementations
///
/// Allows custom encryption/decryption implementations (e.g., HSM, KMS)
pub trait CryptoProvider: Send + Sync {
    /// Encrypts data using the private key
    ///
    /// # Arguments
    /// * `data` - Data to encrypt
    ///
    /// # Returns
    /// URL-safe base64 encoded encrypted data
    fn encrypt_with_private_key(&self, data: &str) -> Result<String>;

    /// Decrypts data using the public key
    ///
    /// # Arguments
    /// * `encrypted_data` - URL-safe base64 encoded encrypted data
    ///
    /// # Returns
    /// Decrypted data string
    fn decrypt_with_public_key(&self, encrypted_data: &str) -> Result<String>;

    /// Signs data using the signing key
    ///
    /// # Arguments
    /// * `data` - Data to sign
    ///
    /// # Returns
    /// Base64 encoded signature
    fn sign(&self, data: &str) -> Result<String>;

    /// Verifies signature
    ///
    /// # Arguments
    /// * `data` - Original data
    /// * `signature` - Base64 encoded signature
    ///
    /// # Returns
    /// True if signature is valid
    fn verify(&self, data: &str, signature: &str) -> Result<bool>;
}

/// RSA Crypto Provider
///
/// Default implementation of CryptoProvider using RSA encryption
/// with segment encryption/decryption for long data.
///
/// Matches the Java SDK's RSAHelper implementation.
#[derive(Clone)]
pub struct RsaCryptoProvider {
    private_key: Option<RsaPrivateKey>,
    public_key: Option<RsaPublicKey>,
    sign_private_key: Option<RsaPrivateKey>,
}

impl RsaCryptoProvider {
    /// Maximum encrypt block size (key_size - 11 for PKCS1 padding)
    /// Java SDK uses 234 bytes
    #[allow(dead_code)]
    const MAX_ENCRYPT_BLOCK: usize = 234;
    /// Maximum decrypt block size (key_size)
    #[allow(dead_code)]
    const MAX_DECRYPT_BLOCK: usize = 256;

    /// Creates a new RSA crypto provider
    ///
    /// # Arguments
    /// * `private_key` - RSA private key in PEM format
    /// * `public_key` - RSA public key in PEM format
    /// * `sign_private_key` - RSA private key for signing (optional)
    pub fn new(
        private_key: Option<&str>,
        public_key: Option<&str>,
        sign_private_key: Option<&str>,
    ) -> Result<Self> {
        let private_key = if let Some(key) = private_key {
            let formatted = Self::format_rsa_key(key, "private");
            Some(
                RsaPrivateKey::from_pkcs8_pem(&formatted)
                    .map_err(|e| CryptoError::new(format!("Failed to parse private key: {}", e)))?,
            )
        } else {
            None
        };

        let public_key = if let Some(key) = public_key {
            let formatted = Self::format_rsa_key(key, "public");
            Some(
                RsaPublicKey::from_public_key_pem(&formatted)
                    .map_err(|e| CryptoError::new(format!("Failed to parse public key: {}", e)))?,
            )
        } else {
            None
        };

        let sign_private_key = if let Some(key) = sign_private_key {
            let formatted = Self::format_rsa_key(key, "private");
            Some(RsaPrivateKey::from_pkcs8_pem(&formatted).map_err(|e| {
                CryptoError::new(format!("Failed to parse sign private key: {}", e))
            })?)
        } else {
            None
        };

        Ok(Self {
            private_key,
            public_key,
            sign_private_key,
        })
    }

    /// Creates a provider with only private and public keys
    pub fn with_keys(private_key: &str, public_key: &str) -> Result<Self> {
        Self::new(Some(private_key), Some(public_key), None)
    }

    /// Formats RSA key to proper PEM format
    /// Supports multiple input formats:
    /// - Standard PEM with headers (-----BEGIN PRIVATE KEY-----)
    /// - Raw base64 without headers
    /// - Base64 with or without line breaks
    fn format_rsa_key(key: &str, key_type: &str) -> String {
        let mut key = key.trim().to_string();

        // Check if already in valid PEM format
        let is_private = key.contains("-----BEGIN PRIVATE KEY-----")
            || key.contains("-----BEGIN RSA PRIVATE KEY-----");
        let is_public = key.contains("-----BEGIN PUBLIC KEY-----")
            || key.contains("-----BEGIN RSA PUBLIC KEY-----");

        // If already has proper headers and content looks formatted, return as-is
        if (key_type == "private" && is_private) || (key_type == "public" && is_public) {
            // Check if it has proper line breaks (PEM should have ~64 char lines)
            if key.contains('\n') {
                return key;
            }
        }

        // Remove existing headers/footers if present
        let headers = [
            "-----BEGIN PRIVATE KEY-----",
            "-----END PRIVATE KEY-----",
            "-----BEGIN RSA PRIVATE KEY-----",
            "-----END RSA PRIVATE KEY-----",
            "-----BEGIN PUBLIC KEY-----",
            "-----END PUBLIC KEY-----",
            "-----BEGIN RSA PUBLIC KEY-----",
            "-----END RSA PUBLIC KEY-----",
        ];

        for header in headers {
            key = key.replace(header, "");
        }

        // Remove all whitespace to get pure base64
        key = key
            .replace('\n', "")
            .replace('\r', "")
            .replace(' ', "")
            .replace('\t', "");

        // Format base64 with line breaks every 64 characters for proper PEM format
        let formatted_base64: String = key
            .chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        // Add proper headers/footers
        if key_type == "private" {
            format!(
                "-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----",
                formatted_base64
            )
        } else {
            format!(
                "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
                formatted_base64
            )
        }
    }

    /// Gets the key size in bytes
    #[allow(dead_code)]
    fn key_size_bytes(&self) -> usize {
        if let Some(ref key) = self.private_key {
            key.size()
        } else if let Some(ref key) = self.public_key {
            key.size()
        } else {
            256 // Default to 2048-bit key
        }
    }

    /// Raw RSA encrypt with private key (non-standard, for signature-like encryption)
    fn raw_encrypt_with_private_key(&self, data: &[u8]) -> Result<Vec<u8>> {
        let private_key = self
            .private_key
            .as_ref()
            .ok_or_else(|| CryptoError::new("Private key is not set"))?;

        let key_size = private_key.size();
        let max_block = key_size - 11;

        let mut encrypted_chunks = Vec::new();
        let mut offset = 0;
        let input_len = data.len();

        while offset < input_len {
            let block_size = std::cmp::min(max_block, input_len - offset);
            let chunk = &data[offset..offset + block_size];

            // Add PKCS#1 v1.5 padding manually
            // Format: 0x00 0x01 [0xFF padding] 0x00 [data]
            let padding_len = key_size - chunk.len() - 3;
            let mut padded = vec![0x00, 0x01];
            padded.extend(vec![0xFF; padding_len]);
            padded.push(0x00);
            padded.extend_from_slice(chunk);

            // Raw RSA operation with private key: m^d mod n
            let padded_int = BigUint::from_bytes_be(&padded);
            let encrypted_int = padded_int.modpow(private_key.d(), private_key.n());
            let mut encrypted_bytes = encrypted_int.to_bytes_be();

            // Ensure the output is key_size bytes
            while encrypted_bytes.len() < key_size {
                encrypted_bytes.insert(0, 0);
            }

            encrypted_chunks.push(encrypted_bytes);
            offset += block_size;
        }

        // Combine all encrypted chunks
        Ok(encrypted_chunks.concat())
    }

    /// Raw RSA decrypt with public key (non-standard, for signature-like decryption)
    fn raw_decrypt_with_public_key(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        let public_key = self
            .public_key
            .as_ref()
            .ok_or_else(|| CryptoError::new("Public key is not set"))?;

        let key_size = public_key.size();
        let mut decrypted_chunks = Vec::new();
        let mut offset = 0;
        let input_len = encrypted_data.len();

        while offset < input_len {
            let block_size = std::cmp::min(key_size, input_len - offset);
            let chunk = &encrypted_data[offset..offset + block_size];

            // Raw RSA operation with public key: c^e mod n
            let encrypted_int = BigUint::from_bytes_be(chunk);
            let decrypted_int = encrypted_int.modpow(public_key.e(), public_key.n());
            let decrypted_bytes = decrypted_int.to_bytes_be();

            // Pad to key_size for proper PKCS#1 parsing
            let mut padded_decrypted = vec![0u8; key_size - decrypted_bytes.len()];
            padded_decrypted.extend_from_slice(&decrypted_bytes);

            // Remove PKCS#1 v1.5 padding: 0x00 0x01 [padding 0xFF...] 0x00 [data]
            // or for encryption: 0x00 0x02 [random padding] 0x00 [data]
            if padded_decrypted.len() >= 11 {
                // Find the 0x00 separator after padding
                let mut separator_idx = None;
                if padded_decrypted.len() >= 2
                    && (padded_decrypted[1] == 0x01 || padded_decrypted[1] == 0x02)
                {
                    for i in 2..padded_decrypted.len() {
                        if padded_decrypted[i] == 0 {
                            separator_idx = Some(i);
                            break;
                        }
                    }
                }

                if let Some(idx) = separator_idx {
                    decrypted_chunks.push(padded_decrypted[idx + 1..].to_vec());
                } else {
                    // No valid padding found, might be raw data
                    let data: Vec<u8> = padded_decrypted
                        .into_iter()
                        .skip_while(|&b| b == 0)
                        .collect();
                    decrypted_chunks.push(data);
                }
            } else {
                let data: Vec<u8> = padded_decrypted
                    .into_iter()
                    .skip_while(|&b| b == 0)
                    .collect();
                decrypted_chunks.push(data);
            }

            offset += block_size;
        }

        // Combine all decrypted chunks
        Ok(decrypted_chunks.concat())
    }
}

impl CryptoProvider for RsaCryptoProvider {
    fn encrypt_with_private_key(&self, data: &str) -> Result<String> {
        let data_bytes = data.as_bytes();
        let encrypted = self.raw_encrypt_with_private_key(data_bytes)?;

        // Convert to URL-safe base64
        Ok(URL_SAFE_NO_PAD.encode(&encrypted))
    }

    fn decrypt_with_public_key(&self, encrypted_data: &str) -> Result<String> {
        // Convert URL-safe base64 back to bytes (handle missing padding)
        let mut padded_data = encrypted_data.to_string();
        let padding = 4 - (padded_data.len() % 4);
        if padding < 4 {
            padded_data.push_str(&"=".repeat(padding));
        }

        let encrypted_bytes = URL_SAFE_NO_PAD
            .decode(encrypted_data)
            .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(&padded_data))
            .map_err(|e| CryptoError::new(format!("Failed to decode base64: {}", e)))?;

        let decrypted = self.raw_decrypt_with_public_key(&encrypted_bytes)?;

        String::from_utf8(decrypted)
            .map_err(|e| CryptoError::new(format!("Failed to decode UTF-8: {}", e)).into())
    }

    fn sign(&self, data: &str) -> Result<String> {
        // Use sign_private_key if set, otherwise fall back to private_key
        let signing_key = self
            .sign_private_key
            .as_ref()
            .or(self.private_key.as_ref())
            .ok_or_else(|| CryptoError::new("Neither sign_private_key nor private_key is set"))?;

        // Step 1: MD5 hash the data (matching Go SDK implementation)
        let mut md5_hasher = Md5::new();
        md5_hasher.update(data.as_bytes());
        let md5_hash = format!("{:x}", md5_hasher.finalize());

        // Step 2: SHA256 hash the MD5 hex string
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(md5_hash.as_bytes());
        let hash = sha256_hasher.finalize();

        // Step 3: RSA sign with PKCS1v15 using SHA256
        // Use Pkcs1v15Sign::new_unprefixed() with SHA256 DigestInfo prefix
        // SHA256 DigestInfo prefix: 30 31 30 0d 06 09 60 86 48 01 65 03 04 02 01 05 00 04 20
        let padding = Pkcs1v15Sign::new_unprefixed();

        // Build DigestInfo structure for SHA256: prefix + hash
        let sha256_prefix: [u8; 19] = [
            0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
            0x01, 0x05, 0x00, 0x04, 0x20,
        ];
        let mut digest_info = Vec::with_capacity(sha256_prefix.len() + hash.len());
        digest_info.extend_from_slice(&sha256_prefix);
        digest_info.extend_from_slice(&hash);

        let signature = signing_key
            .sign(padding, &digest_info)
            .map_err(|e| CryptoError::new(format!("Failed to sign: {}", e)))?;

        // Step 4: Return Base64 encoded signature
        Ok(base64::engine::general_purpose::STANDARD.encode(signature))
    }

    fn verify(&self, data: &str, signature: &str) -> Result<bool> {
        let public_key = self
            .public_key
            .as_ref()
            .ok_or_else(|| CryptoError::new("Public key is not set"))?;

        let signature_bytes = base64::engine::general_purpose::STANDARD
            .decode(signature)
            .map_err(|e| CryptoError::new(format!("Failed to decode signature: {}", e)))?;

        let verifying_key = VerifyingKey::<Sha256>::new_unprefixed(public_key.clone());
        let signature = Signature::try_from(signature_bytes.as_slice())
            .map_err(|e| CryptoError::new(format!("Invalid signature format: {}", e)))?;

        Ok(verifying_key.verify(data.as_bytes(), &signature).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_rsa_key() {
        let raw_key = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA";
        let formatted = RsaCryptoProvider::format_rsa_key(raw_key, "public");
        assert!(formatted.starts_with("-----BEGIN PUBLIC KEY-----"));
        assert!(formatted.ends_with("-----END PUBLIC KEY-----"));
    }
}
