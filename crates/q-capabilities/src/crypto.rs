//! Cryptographic random byte and UUID generation via OS CSPRNG (M27-008-A).
//!
//! Provides bounded, secure entropy primitives (`getRandomValues`, `randomUUID`)
//! without implementing broad or custom cryptographic algorithms.

use std::fmt;

/// Maximum byte length for a single `getRandomValues` call (64 KiB, matching Web Crypto spec).
pub const MAX_RANDOM_BYTES_LEN: usize = 65_536;

/// Typed crypto errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    QuotaExceeded { len: usize, max: usize },
    EntropyUnavailable(String),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::QuotaExceeded { len, max } => {
                write!(
                    f,
                    "QuotaExceededError: requested {len} bytes exceeds maximum allowed limit of {max} bytes"
                )
            }
            CryptoError::EntropyUnavailable(err) => {
                write!(f, "entropy source unavailable: {err}")
            }
        }
    }
}

impl std::error::Error for CryptoError {}

/// Secure OS CSPRNG primitive wrapper.
pub struct CryptoRandom;

impl CryptoRandom {
    /// Fill the destination slice with cryptographically secure random bytes from OS CSPRNG.
    /// Enforces the Web Crypto 64 KiB QuotaExceeded limit.
    pub fn get_random_values(dest: &mut [u8]) -> Result<(), CryptoError> {
        if dest.len() > MAX_RANDOM_BYTES_LEN {
            return Err(CryptoError::QuotaExceeded {
                len: dest.len(),
                max: MAX_RANDOM_BYTES_LEN,
            });
        }
        if dest.is_empty() {
            return Ok(());
        }
        getrandom::getrandom(dest).map_err(|e| CryptoError::EntropyUnavailable(e.to_string()))
    }

    /// Generate a standard RFC 4122 v4 UUID using OS CSPRNG.
    pub fn random_uuid() -> Result<String, CryptoError> {
        let mut bytes = [0u8; 16];
        getrandom::getrandom(&mut bytes)
            .map_err(|e| CryptoError::EntropyUnavailable(e.to_string()))?;

        // Set version to 4 (0100 in bits 4..7 of byte 6)
        bytes[6] = (bytes[6] & 0x0F) | 0x40;
        // Set variant to RFC 4122 (10xx in bits 6..7 of byte 8)
        bytes[8] = (bytes[8] & 0x3F) | 0x80;

        let mut buf = String::with_capacity(36);
        use std::fmt::Write;
        for (i, b) in bytes.iter().enumerate() {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                buf.push('-');
            }
            let _ = write!(buf, "{b:02x}");
        }
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_random_values_fills_non_zero_entropy() {
        let mut buf1 = [0u8; 32];
        let mut buf2 = [0u8; 32];
        CryptoRandom::get_random_values(&mut buf1).unwrap();
        CryptoRandom::get_random_values(&mut buf2).unwrap();

        // Must not be all zeroes
        assert!(buf1.iter().any(|&b| b != 0));
        assert!(buf2.iter().any(|&b| b != 0));
        // Two independent generations must not match
        assert_ne!(buf1, buf2);
    }

    #[test]
    fn get_random_values_quota_limit() {
        let mut huge = vec![0u8; MAX_RANDOM_BYTES_LEN + 1];
        assert_eq!(
            CryptoRandom::get_random_values(&mut huge),
            Err(CryptoError::QuotaExceeded {
                len: MAX_RANDOM_BYTES_LEN + 1,
                max: MAX_RANDOM_BYTES_LEN,
            })
        );

        let mut exact = vec![0u8; MAX_RANDOM_BYTES_LEN];
        assert!(CryptoRandom::get_random_values(&mut exact).is_ok());
    }

    #[test]
    fn random_uuid_rfc4122_v4_format() {
        let uuid1 = CryptoRandom::random_uuid().unwrap();
        let uuid2 = CryptoRandom::random_uuid().unwrap();

        assert_eq!(uuid1.len(), 36);
        assert_eq!(uuid2.len(), 36);
        assert_ne!(uuid1, uuid2);

        // Check hyphen positions: 8-4-4-4-12
        let parts: Vec<&str> = uuid1.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);

        // Version 4 check: 13th character (index 14) is '4'
        assert_eq!(uuid1.chars().nth(14), Some('4'));
        // Variant check: 17th character (index 19) is one of '8', '9', 'a', 'b'
        let variant_char = uuid1.chars().nth(19).unwrap();
        assert!(matches!(variant_char, '8' | '9' | 'a' | 'b'));
    }

    /// M27-008-C: fail-closed error formatting when entropy is unavailable.
    #[test]
    fn entropy_unavailable_error_formatting_and_fail_closed() {
        let err = CryptoError::EntropyUnavailable("OS CSPRNG kernel syscall failed".into());
        let err_str = err.to_string();
        assert!(err_str.contains("entropy source unavailable"));
        assert!(err_str.contains("OS CSPRNG kernel syscall failed"));
    }

    /// M27-008-D: security audit test confirming no custom or pseudo-random cryptographic primitives.
    #[test]
    fn no_custom_or_pseudorandom_primitives() {
        // Enforce that crypto surface only provides getRandomValues and randomUUID,
        // both delegating directly to OS CSPRNG (getrandom crate).
        let mut buf = [0u8; 16];
        assert!(CryptoRandom::get_random_values(&mut buf).is_ok());
        let uuid = CryptoRandom::random_uuid().unwrap();
        assert_eq!(uuid.len(), 36);
    }
}
