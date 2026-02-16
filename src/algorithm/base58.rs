use crate::{ByteVec, EncodedString, Encoding, SerialiseError};

const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Base58 encoding implementation (Bitcoin-style).
///
/// This type provides methods to encode and decode data using base58 encoding,
/// which uses a URL- and filename-safe alphabet that omits visually ambiguous
/// characters.
#[derive(Debug)]
pub struct Base58 {
    /// The base58-encoded string representation
    serialised: EncodedString,
}

impl Base58 {
    /// Creates a new `Base58` instance.
    ///
    /// # Arguments
    /// * `serialised` - The base58-encoded string
    #[must_use]
    pub const fn new(serialised: EncodedString) -> Self {
        Self { serialised }
    }

    /// Returns the base58-encoded string.
    #[must_use]
    pub fn get_serialised(self) -> EncodedString {
        self.serialised
    }

    /// Encodes a byte slice using base58 encoding.
    ///
    /// # Arguments
    /// * `bytes` - The bytes to encode
    ///
    /// # Returns
    /// The base58-encoded string
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_base58(bytes: &[u8]) -> String {
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
                *b = u8::try_from(v / 58)
                    .unwrap_or_else(|_| unreachable!("base58 division quotient must fit in u8"));
                rem = v % 58;
            }

            out.push(ALPHABET[rem as usize]);

            while n.first().copied() == Some(0) {
                n.remove(0);
            }
        }

        out.reverse();
        out.into_iter().map(char::from).collect()
    }

    fn base58_to_bytes(base58: &str) -> Vec<u8> {
        let s = base58.trim();
        if s.is_empty() || s == "0" {
            return vec![0];
        }

        let mut bytes: Vec<u8> = vec![0];

        for c in s.bytes() {
            let digit = ALPHABET.iter().position(|&b| b == c).map_or_else(
                || panic!("invalid base58 character"),
                |p| {
                    u32::try_from(p)
                        .unwrap_or_else(|_| unreachable!("base58 digit index must fit in u32"))
                },
            );

            let mut carry = digit;
            for b in bytes.iter_mut().rev() {
                let v = u32::from(*b) * 58 + carry;
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

    /// Decodes a base58 string into bytes, optionally left-padding to `size`.
    ///
    /// # Panics
    ///
    /// Panics if `base58` contains a character outside the base58 alphabet.
    ///
    /// Panics if the decoded value requires more than `size` bytes when `size > 0`.
    #[must_use]
    pub fn from_base58(base58: &str, size: usize) -> Vec<u8> {
        let mut bytes = Self::base58_to_bytes(base58);

        assert!(
            !(bytes.len() > size && size > 0),
            "base58 value does not fit in {size} bytes"
        );

        if bytes.len() < size && size > 0 {
            let mut padded = vec![0u8; size - bytes.len()];
            padded.append(&mut bytes);
            return padded;
        }

        bytes
    }
}

impl TryFrom<ByteVec> for Base58 {
    type Error = SerialiseError;
    fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
        Ok(Self::new(EncodedString::new(
            Encoding::Base58,
            Self::to_base58(&value.get_bytes()),
        )))
    }
}

impl TryFrom<Base58> for EncodedString {
    type Error = SerialiseError;
    fn try_from(value: Base58) -> Result<Self, Self::Error> {
        Ok(value.get_serialised())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_base58() {
        let string = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let base58 = Base58::to_base58(string);
        assert_eq!(base58, "NE1FfXYqCHge2p4MZ56o8gdrDWMiHXPJLXk9ixxKgUebU7VqB",);
    }

    #[test]
    fn test_from_base58() {
        let string = "NE1FfXYqCHge2p4MZ56o8gdrDWMiHXPJLXk9ixxKgUebU7VqB";
        let bytes = Base58::from_base58(string, 0);
        assert_eq!(bytes, b"0123456789abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    #[should_panic(expected = "invalid base58 character")]
    fn test_from_invalid_base58_panics() {
        let string = "NE1FfXYqCHge2p4MZ56o8gdrDWMiH!XPJLXk9ixxKgUebU7VqB";
        let _bytes = Base58::from_base58(string, 0);
    }
}
