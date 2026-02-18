use crate::{ByteVec, EncodedString, Encoding, SerialiseError};

const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

/// Base36 encoding implementation (0-9 and A-Z).
///
/// This type provides methods to encode and decode data using base36 encoding,
/// which uses the digits 0-9 and letters A-Z to represent data.
#[derive(Debug)]
pub struct Base36 {
    /// The base36-encoded string representation
    serialised: EncodedString,
}

impl Base36 {
    /// Creates a new `Base36` instance.
    ///
    /// # Arguments
    /// * `serialised` - The base36-encoded string
    #[must_use = "This creates a new Base36 instance but does nothing if unused"]
    pub const fn new(serialised: EncodedString) -> Self {
        Self { serialised }
    }

    /// Returns the base36-encoded string.
    #[must_use = "This returns the encoded string but does nothing if unused"]
    pub fn get_serialised(self) -> EncodedString {
        self.serialised
    }

    /// Encodes a byte slice using base36 encoding.
    ///
    /// # Arguments
    /// * `bytes` - The bytes to encode
    ///
    /// # Returns
    /// The base36-encoded string
    #[must_use = "This returns the encoded string and does nothing if unused"]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_base36(bytes: &[u8]) -> String {
        if bytes.is_empty() || bytes.iter().all(|&b| b == 0) {
            return "0".to_string();
        }

        let mut n: Vec<u8> = bytes.to_vec();
        let mut out = Vec::new();

        while !n.is_empty() {
            let mut rem = 0;
            let mut i = 0;

            while i < n.len() {
                let v = u32::from(n[i]) + (rem * 256);
                n[i] = u8::try_from(v / 36).unwrap_or_else(|_| unreachable!());
                rem = v % 36;
                i += 1;
            }

            out.push(u8::try_from(rem).unwrap_or_else(|_| unreachable!()));

            while n.first().copied() == Some(0) {
                n.remove(0);
            }
        }

        let mut result = String::with_capacity(out.len());
        for byte in out.iter().rev() {
            result.push(ALPHABET[*byte as usize] as char);
        }
        result
    }

    /// Converts a base36 string into its byte representation.
    ///
    /// # Arguments
    /// * `base36` - The base36-encoded string to convert
    ///
    /// # Returns
    /// The decoded bytes
    ///
    /// # Errors
    /// Returns `SerialiseError` if the input contains invalid base36 characters
    pub fn base36_to_bytes(base36: &str) -> Result<Vec<u8>, SerialiseError> {
        let s = base36.trim();
        if s.is_empty() || s == "0" {
            return Ok(vec![0]);
        }

        let mut acc = vec![0u8];
        for c in s.chars() {
            let Some(digit_usize) = ALPHABET
                .iter()
                .position(|x| *x == c.to_ascii_lowercase() as u8)
            else {
                return Err(SerialiseError::new("Invalid base36 character".to_string()));
            };
            let digit = u32::from(u8::try_from(digit_usize).unwrap_or_else(|_| unreachable!()));

            let mut carry = digit;
            for b in acc.iter_mut().rev() {
                let v = u32::from(*b) * 36 + carry;
                *b = u8::try_from(v & 0xff).unwrap_or_else(|_| unreachable!());
                carry = v >> 8;
            }

            while carry > 0 {
                acc.insert(
                    0,
                    u8::try_from(carry & 0xff).unwrap_or_else(|_| unreachable!()),
                );
                carry >>= 8;
            }
        }

        while acc.len() > 1 && acc[0] == 0 {
            acc.remove(0);
        }

        Ok(acc)
    }

    /// Decodes a base36 string into bytes, optionally left-padding to `size`.
    ///
    /// # Arguments
    /// * `base36` - The base36-encoded string to decode
    /// * `size` - The expected size of the output in bytes. If greater than 0,
    ///   the output will be padded or truncated to this size.
    ///
    /// # Returns
    /// The decoded bytes
    ///
    /// # Errors
    /// Returns `Err` if `base36` contains characters outside the base36 alphabet.
    /// Returns `Err` if the decoded value requires more than `size` bytes when `size > 0`
    #[must_use = "This returns the decoded bytes and does nothing if unused"]
    pub fn from_base36(base36: &str, size: usize) -> Result<Vec<u8>, SerialiseError> {
        match Self::base36_to_bytes(base36) {
            Err(e) => Err(e),
            Ok(mut bytes) => {
                if bytes.len() > size && size > 0 {
                    return Err(SerialiseError::new(format!(
                        "base36 value does not fit in {size} bytes"
                    )));
                }

                if bytes.len() < size && size > 0 {
                    let mut padded = vec![0u8; size - bytes.len()];
                    padded.append(&mut bytes);
                    return Ok(padded);
                }

                Ok(bytes)
            }
        }
    }
}

impl TryFrom<ByteVec> for Base36 {
    type Error = SerialiseError;
    fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
        Ok(Self::new(EncodedString::new(
            Encoding::Base36,
            Self::to_base36(value.get_bytes()),
        )))
    }
}

impl TryFrom<Base36> for EncodedString {
    type Error = SerialiseError;
    fn try_from(value: Base36) -> Result<Self, Self::Error> {
        Ok(value.get_serialised())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_base36() {
        let string = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let base36 = Base36::to_base36(string);
        assert_eq!(
            base36,
            "2dbg0rhouyms2hsh4jiluolq0rx1et8yty277nr9mwq20b47cwxc2id6"
        );
    }

    #[test]
    fn test_from_base36() {
        const NO_MATCH: &[u8] = b"no match";
        let string = "2dbg0rhouyms2hsh4jiluolq0rx1et8yty277nr9mwq20b47cwxc2id6";
        let bytes = Base36::from_base36(string, 0);
        assert!(bytes.is_ok());
        assert_eq!(
            bytes.unwrap_or_else(|_| NO_MATCH.to_vec()),
            b"0123456789abcdefghijklmnopqrstuvwxyz"
        );
    }

    #[test]
    fn test_from_invalid_base36() {
        let string = "2dbg0rhouyms2hsh4jiluolq0rx!1et8yty277nr9mwq20b47cwxc2id6";
        let bytes = Base36::from_base36(string, 0);
        if let Err(e) = bytes {
            assert_eq!(e.to_string(), "Invalid base36 character".to_string());
        } else {
            panic!("should have failed");
        }
    }
}
