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
    #[must_use]
    pub fn to_hex(bytes: &[u8]) -> String {
        let mut out: Vec<u8> = Vec::with_capacity(bytes.len() * 2);
        for &b in bytes {
            out.push(ALPHABET[(b >> 4) as usize]);
            out.push(ALPHABET[(b & 0x0f) as usize]);
        }

        // `out` is guaranteed to be ASCII.
        unsafe { String::from_utf8_unchecked(out) }
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
    /// # Panics
    ///
    /// Panics if `hex` contains a non-hex character.
    ///
    /// Panics if `hex` contains an odd number of characters.
    #[must_use]
    pub fn from_hex(hex: &str) -> Vec<u8> {
        let s = hex.trim();
        if s.is_empty() {
            return vec![];
        }

        assert!(
            s.len().is_multiple_of(2),
            "hex string must have an even length"
        );

        let mut out: Vec<u8> = Vec::with_capacity(s.len() / 2);
        let bytes = s.as_bytes();
        for i in (0..bytes.len()).step_by(2) {
            let Some(hi) = Self::from_hex_digit(bytes[i]) else {
                panic!("invalid hex character");
            };
            let Some(lo) = Self::from_hex_digit(bytes[i + 1]) else {
                panic!("invalid hex character");
            };
            out.push((hi << 4) | lo);
        }
        out
    }
}

impl TryFrom<Hex> for Vec<u8> {
    type Error = SerialiseError;
    fn try_from(value: Hex) -> Result<Self, Self::Error> {
        Ok(Hex::from_hex(value.get_serialised().get_string()))
    }
}

impl TryFrom<Vec<u8>> for Hex {
    type Error = SerialiseError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::new(EncodedString::new(
            Encoding::Hex,
            Self::to_hex(&value),
        )))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_hex() {
        let bytes = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let hex = Hex::to_hex(bytes);
        assert_eq!(
            hex,
            "303132333435363738396162636465666768696a6b6c6d6e6f707172737475767778797a"
        );
    }

    #[test]
    fn test_from_hex() {
        let string = "303132333435363738396162636465666768696a6b6c6d6e6f707172737475767778797a";
        let bytes = Hex::from_hex(string);
        assert_eq!(bytes, b"0123456789abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    #[should_panic(expected = "invalid hex character")]
    fn test_from_invalid_hex_panics() {
        let string = "gg";
        let _bytes = Hex::from_hex(string);
    }
}
