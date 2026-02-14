use crate::serialise::{Base36, EncodedString, Encoding, SerialiseError, algorithm::Base58};

/// Raw byte representation of serializable data.
///
/// This type represents the raw bytes of a serializable structure along with
/// its type information. It serves as an intermediate format between the
/// original data and its string representation.
#[derive(Debug, Clone)]
pub struct ByteVec {
    bytes: Vec<u8>,
}

impl ByteVec {
    /// Creates a new `ByteVec` instance.
    ///
    /// # Arguments
    /// * `bytes` - The raw byte data
    #[must_use = "This creates a new ByteVec instance but does nothing if unused"]
    pub const fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Returns the raw byte data.
    #[must_use = "This returns the byte data but does nothing if unused"]
    pub fn get_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Attempts to convert these bytes into a string representation.
    ///
    /// # Arguments
    /// * `serialise_type` - The format to use for serialization
    ///
    /// # Returns
    /// The serialized string representation
    ///
    /// # Errors
    /// Returns `SerialiseError` if:
    /// - The serialization format is not supported
    /// - The bytes cannot be encoded in the specified format
    #[must_use = "This returns a Result that must be handled"]
    pub fn try_encode(self, encoding: Encoding) -> Result<EncodedString, SerialiseError> {
        match encoding {
            Encoding::Base36 => self.try_encode_base36(),
            Encoding::Base58 => self.try_encode_base58(),
            _ => Err(SerialiseError::new("Unsupported encoding".to_string())),
        }
    }

    /// Attempts to convert these bytes into a base36-encoded string.
    ///
    /// # Returns
    /// The base36-encoded string representation
    ///
    /// # Errors
    /// Returns `SerialiseError` if:
    /// - The bytes cannot be encoded in base36 format
    /// - The Base36 conversion fails
    #[must_use = "This returns a Result that must be handled"]
    pub fn try_encode_base36(self) -> Result<EncodedString, SerialiseError> {
        match Base36::try_from(self) {
            Ok(base36) => match base36.try_into() {
                Ok(serialstring) => Ok(serialstring),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        }
    }

    /// Attempts to convert these bytes into a base58-encoded string.
    ///
    /// # Returns
    /// The base58-encoded string representation
    ///
    /// # Errors
    /// Returns `SerialiseError` if:
    /// - The bytes cannot be encoded in base58 format
    /// - The Base58 conversion fails
    #[must_use = "This returns a Result that must be handled"]
    pub fn try_encode_base58(self) -> Result<EncodedString, SerialiseError> {
        match Base58::try_from(self) {
            Ok(base58) => match base58.try_into() {
                Ok(serialstring) => Ok(serialstring),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_base36() {
        let bytes = ByteVec::new(b"0123456789abcdefghijklmnopqrstuvwxyz".to_vec());
        let encoded = bytes.try_encode(Encoding::Base36);
        assert!(encoded.is_ok());
        assert_eq!(
            encoded
                .unwrap_or_else(|_| EncodedString::new(Encoding::Base36, "no match".to_string()))
                .get_string(),
            "2dbg0rhouyms2hsh4jiluolq0rx1et8yty277nr9mwq20b47cwxc2id6"
        );
    }

    #[test]
    fn test_encoding_base58() {
        let bytes = ByteVec::new(b"0123456789abcdefghijklmnopqrstuvwxyz".to_vec());
        let encoded = bytes.try_encode(Encoding::Base58);
        assert!(encoded.is_ok());
        assert_eq!(
            encoded
                .unwrap_or_else(|_| EncodedString::new(Encoding::Base36, "no match".to_string()))
                .get_string(),
            "NE1FfXYqCHge2p4MZ56o8gdrDWMiHXPJLXk9ixxKgUebU7VqB"
        );
    }
}
