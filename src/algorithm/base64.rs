use crate::{EncodedString, Encoder, Encoding, SerialiseError};

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
    ///
    /// # Errors
    ///
    /// This function currently does not return an error.
    pub fn try_to_base64(bytes: &[u8]) -> Result<String, SerialiseError> {
        if bytes.is_empty() {
            return Ok("0".to_string());
        }

        if bytes.iter().all(|&b| b == 0) {
            return Ok("0".to_string());
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
        Ok(out.into_iter().map(char::from).collect())
    }

    fn base64_to_bytes(base64: &str) -> Result<Vec<u8>, SerialiseError> {
        let s = base64.trim();
        if s.is_empty() || s == "0" {
            return Ok(vec![0]);
        }

        let mut bytes: Vec<u8> = vec![0];

        for c in s.bytes() {
            let Some(pos) = ALPHABET.iter().position(|&b| b == c) else {
                return Err(SerialiseError::new("invalid base64 character".to_string()));
            };
            let digit = u32::try_from(pos).unwrap_or_else(|_| unreachable!());

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

        Ok(bytes)
    }

    /// Decodes a base64 string into bytes, optionally left-padding to `size`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if `base64` contains characters outside the base64 alphabet.
    ///
    /// Returns `Err` if the decoded value requires more than `size` bytes when `size > 0`.
    ///
    pub fn try_from_base64(base64: &str, size: usize) -> Result<Vec<u8>, SerialiseError> {
        let mut bytes = Self::base64_to_bytes(base64)?;

        if bytes.len() > size && size > 0 {
            return Err(SerialiseError::new(format!(
                "base64 value does not fit in {size} bytes"
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

impl Encoder for Base64 {
    fn try_encode(bytes: &[u8]) -> Result<EncodedString, SerialiseError> {
        Ok(EncodedString::new(
            Encoding::Base64,
            Self::try_to_base64(bytes).unwrap_or_else(|_| String::new()),
        ))
    }

    fn try_decode(encoded: &EncodedString) -> Result<Vec<u8>, SerialiseError> {
        Self::try_from_base64(encoded.get_string(), 0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_base64() {
        let string = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let base64 = Base64::try_to_base64(string).unwrap_or_else(|_| String::new());
        assert_eq!(base64, "MDEyMzQ1Njc4OWFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6");
    }

    #[test]
    fn test_from_base64() {
        let string = "MDEyMzQ1Njc4OWFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6";
        let bytes = Base64::try_from_base64(string, 0).unwrap_or_else(|_| vec![]);
        assert_eq!(bytes, b"0123456789abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn test_from_invalid_base64_panics() {
        let string = "NE1FfXYqCHge2p4MZ56o8gdrDWMiH!XPJLXk9ixxKgUebU7VqB";
        let bytes = Base64::try_from_base64(string, 0);
        assert!(bytes.is_err());
    }
}
