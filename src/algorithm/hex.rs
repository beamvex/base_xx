use crate::{EncodedString, Encoding, SerialiseError};

const ALPHABET: &[u8; 16] = b"0123456789abcdef";

/// Hex (base16) encoding implementation (RFC 4648).
#[derive(Debug)]
pub struct Hex {
    serialised: EncodedString,
}

impl Hex {
    /// Create a new Hex instance.
    #[must_use]
    pub const fn new(serialised: EncodedString) -> Self {
        Self { serialised }
    }

    /// Get the serialised data.
    #[must_use]
    pub fn get_serialised(self) -> EncodedString {
        self.serialised
    }

    /// Convert bytes to a lowercase hex string.
    ///
    /// # Errors
    ///
    /// This function never returns an error.
    pub fn try_to_hex(bytes: &[u8]) -> Result<String, SerialiseError> {
        let mut out: Vec<u8> = Vec::with_capacity(bytes.len() * 2);
        for &b in bytes {
            out.push(ALPHABET[(b >> 4) as usize]);
            out.push(ALPHABET[(b & 0x0f) as usize]);
        }

        // `out` is guaranteed to be ASCII.
        unsafe { Ok(String::from_utf8_unchecked(out)) }
    }

    const fn from_hex_digit(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(10 + (c - b'a')),
            b'A'..=b'F' => Some(10 + (c - b'A')),
            _ => None,
        }
    }

    /// Decodes a hex string into bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if `hex` contains a non-hex character.
    ///
    /// Returns an error if `hex` contains an odd number of characters.
    pub fn try_from_hex(hex: &str) -> Result<Vec<u8>, SerialiseError> {
        let s = hex.trim();
        if s.is_empty() {
            return Ok(vec![]);
        }

        if !s.len().is_multiple_of(2) {
            return Err(SerialiseError::new(
                "hex string must have an even length".to_string(),
            ));
        }

        let mut out: Vec<u8> = Vec::with_capacity(s.len() / 2);
        let bytes = s.as_bytes();
        for i in (0..bytes.len()).step_by(2) {
            let Some(hi) = Self::from_hex_digit(bytes[i]) else {
                return Err(SerialiseError::new("invalid hex character".to_string()));
            };
            let Some(lo) = Self::from_hex_digit(bytes[i + 1]) else {
                return Err(SerialiseError::new("invalid hex character".to_string()));
            };
            out.push((hi << 4) | lo);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_hex() {
        let bytes = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let hex = Hex::try_to_hex(bytes).unwrap_or_else(|_| String::new());
        assert_eq!(
            hex,
            "303132333435363738396162636465666768696a6b6c6d6e6f707172737475767778797a"
        );
    }

    #[test]
    fn test_from_hex() {
        let string = "303132333435363738396162636465666768696a6b6c6d6e6f707172737475767778797a";
        assert!(matches!(
            Hex::try_from_hex(string),
            Ok(bytes) if bytes == b"0123456789abcdefghijklmnopqrstuvwxyz"
        ));
    }

    #[test]
    fn test_from_invalid_hex_is_err() {
        let string = "gg";
        assert!(Hex::try_from_hex(string).is_err());
    }
}
