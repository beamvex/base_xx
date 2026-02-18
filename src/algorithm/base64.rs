use crate::{EncodedString, Encoding, SerialiseError};

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Base64 encoding implementation (RFC 4648).
#[derive(Debug)]
pub struct Base64 {
    serialised: EncodedString,
}

impl Base64 {
    /// Create a new Base64 instance.
    #[must_use]
    pub const fn new(serialised: EncodedString) -> Self {
        Self { serialised }
    }

    /// Get the serialised data.
    #[must_use]
    pub fn get_serialised(self) -> EncodedString {
        self.serialised
    }

    /// Convert bytes to base64 string.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_base64(bytes: &[u8]) -> String {
        if bytes.is_empty() {
            return "0".to_string();
        }

        if bytes.iter().all(|&b| b == 0) {
            return "0".to_string();
        }

        let mut n = bytes.to_vec();
        let mut out: Vec<u8> = Vec::new();

        while !n.is_empty() && n.iter().any(|&b| b != 0) {
            let mut rem: u32 = 0;
            for b in &mut n {
                let v = (rem << 8) | u32::from(*b);
                *b = u8::try_from(v / 64).unwrap_or_else(|_| unreachable!());
                rem = v % 64;
            }

            out.push(ALPHABET[rem as usize]);

            while n.first().copied() == Some(0) {
                n.remove(0);
            }
        }

        out.reverse();
        out.into_iter().map(char::from).collect()
    }

    fn base64_to_bytes(base64: &str) -> Vec<u8> {
        let s = base64.trim();
        if s.is_empty() || s == "0" {
            return vec![0];
        }

        let mut bytes: Vec<u8> = vec![0];

        for c in s.bytes() {
            let digit = ALPHABET.iter().position(|&b| b == c).map_or_else(
                || panic!("invalid base64 character"),
                |p| u32::try_from(p).unwrap_or_else(|_| unreachable!()),
            );

            let mut carry = digit;
            for b in bytes.iter_mut().rev() {
                let v = u32::from(*b) * 64 + carry;
                *b = (v & 0xff) as u8;
                carry = v >> 8;
            }

            while carry > 0 {
                bytes.insert(0, (carry & 0xff) as u8);
                carry >>= 8;
            }
        }

        while bytes.len() > 1 && bytes[0] == 0 {
            bytes.remove(0);
        }

        bytes
    }

    /// Decodes a base64 string into bytes, optionally left-padding to `size`.
    ///
    /// # Panics
    ///
    /// Panics if `base64` contains a character outside the base64 alphabet.
    ///
    /// Panics if the decoded value requires more than `size` bytes when `size > 0`.
    #[must_use]
    pub fn from_base64(base64: &str, size: usize) -> Vec<u8> {
        let mut bytes = Self::base64_to_bytes(base64);

        assert!(
            !(bytes.len() > size && size > 0),
            "base64 value does not fit in {size} bytes"
        );

        if bytes.len() < size && size > 0 {
            let mut padded = vec![0u8; size - bytes.len()];
            padded.append(&mut bytes);
            return padded;
        }

        bytes
    }
}

impl TryFrom<Base64> for Vec<u8> {
    type Error = SerialiseError;
    fn try_from(value: Base64) -> Result<Self, Self::Error> {
        Ok(Base64::from_base64(value.get_serialised().get_string(), 0))
    }
}

impl TryFrom<Vec<u8>> for Base64 {
    type Error = SerialiseError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::new(EncodedString::new(
            Encoding::Base64,
            Self::to_base64(&value),
        )))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_base64() {
        let string = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let base64 = Base64::to_base64(string);
        assert_eq!(base64, "MDEyMzQ1Njc4OWFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6");
    }

    #[test]
    fn test_from_base64() {
        let string = "MDEyMzQ1Njc4OWFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6";
        let bytes = Base64::from_base64(string, 0);
        assert_eq!(bytes, b"0123456789abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    #[should_panic(expected = "invalid base64 character")]
    fn test_from_invalid_base64_panics() {
        let string = "NE1FfXYqCHge2p4MZ56o8gdrDWMiH!XPJLXk9ixxKgUebU7VqB";
        let _bytes = Base64::from_base64(string, 0);
    }
}
