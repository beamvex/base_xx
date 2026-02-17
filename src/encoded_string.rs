use crate::{Base36, ByteVec, Encoding, SerialiseError};

/// String representation of serialized data.
///
/// This type represents data that has been serialized into a string format,
/// along with information about which serialization format was used.
#[derive(Debug, Clone)]
pub struct EncodedString {
    /// The format used to serialize the data
    encoding: Encoding,
    /// The serialized string representation
    string: String,
}

impl EncodedString {
    /// Creates a new `SerialString` instance.
    ///
    /// # Arguments
    /// * `serialise_type` - The format used to serialize the data
    /// * `string` - The serialized string representation
    #[must_use = "This creates a new SerialString instance but does nothing if unused"]
    pub const fn new(encoding: Encoding, string: String) -> Self {
        Self { encoding, string }
    }

    /// Returns the format used to serialize the data.
    ///
    /// # Returns
    /// The serialization format.
    #[must_use = "This returns the serialization format but does nothing if unused"]
    pub const fn get_encoding(&self) -> Encoding {
        self.encoding
    }

    /// Returns the serialized string representation.
    ///
    /// # Returns
    /// The serialized string.
    #[must_use = "This returns the serialized string but does nothing if unused"]
    pub const fn get_string(&self) -> &String {
        &self.string
    }

    /// Attempts to decode the serialized string into bytes.
    ///
    /// # Errors
    /// Returns `Err` if the encoding is unsupported or if decoding the string fails.
    #[must_use = "This returns a Result that must be handled"]
    pub fn try_decode(self) -> Result<ByteVec, SerialiseError> {
        match self.encoding {
            Encoding::Base36 => self.try_decode_base36(),
            _ => Err(SerialiseError::new("Unsupported encoding".to_string())),
        }
    }

    /// Attempts to decode a Base36-encoded string into bytes.
    ///
    /// # Errors
    /// Returns `Err` if the underlying Base36 decoding fails.
    pub fn try_decode_base36(self) -> Result<ByteVec, SerialiseError> {
        match Base36::from_base36(self.string.as_str(), 0) {
            Ok(bytes) => Ok(ByteVec::new(bytes)),
            Err(error) => Err(error),
        }
    }
}

impl std::fmt::Display for EncodedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

/// Implements decoding helpers for a type that can be constructed from decoded bytes.
///
/// This macro adds `try_decode` and `try_decode_base36` associated functions to the
/// given type, delegating to [`EncodedString`] and then converting via `TryFrom`.
///
#[macro_export]
macro_rules! decodable {
    ($t:ty) => {
        impl $t {
            /// Attempts to decode an encoded string into this type.
            ///
            /// # Errors
            /// Returns `Err` if the underlying decoding fails.
            #[must_use = "decoding returns a result that must be handled"]
            pub fn try_decode(encoded_string: EncodedString) -> Result<Self, SerialiseError> {
                match encoded_string.get_encoding() {
                    Encoding::Base36 => Self::try_decode_base36(encoded_string),
                    _ => Err(SerialiseError::new("Unsupported encoding".to_string())),
                }
            }

            /// Attempts to decode a base36-encoded string into this type.
            ///
            /// # Errors
            /// Returns `Err` if the underlying decoding fails.
            #[must_use = "decoding returns a result that must be handled"]
            pub fn try_decode_base36(
                encoded_string: EncodedString,
            ) -> Result<Self, SerialiseError> {
                match encoded_string.try_decode_base36() {
                    Ok(bytes) => Self::try_from(bytes),
                    Err(error) => Err(error),
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_base36() {
        let encoded = EncodedString::new(
            Encoding::Base36,
            "2dbg0rhouyms2hsh4jiluolq0rx1et8yty277nr9mwq20b47cwxc2id6".to_string(),
        );
        let decoded = encoded.try_decode();
        assert!(decoded.is_ok());
        assert_eq!(
            decoded.unwrap_or_else(|_| ByteVec::new(vec![])).get_bytes(),
            b"0123456789abcdefghijklmnopqrstuvwxyz"
        );
    }

    #[test]
    fn test_decode_base36_invalid() {
        let encoded = EncodedString::new(
            Encoding::Base36,
            "2dbg0rhouyms2hsh4jiluolq0rx1!et8yty277nr9mwq20b47cwxc2id6".to_string(),
        );
        let decoded = encoded.try_decode();
        assert!(decoded.is_err());
        if let Err(e) = decoded {
            assert_eq!(e.to_string(), "Invalid base36 character");
        }
    }

    #[test]
    fn test_decoable_decode_base36() {
        #[derive(Debug, PartialEq)]
        struct TestType {
            value: Vec<u8>,
        }

        impl TryFrom<ByteVec> for TestType {
            type Error = SerialiseError;

            fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
                Ok(Self {
                    value: value.get_bytes(),
                })
            }
        }

        decodable!(TestType);

        let encoded = EncodedString::new(
            Encoding::Base36,
            "2dbg0rhouyms2hsh4jiluolq0rx1et8yty277nr9mwq20b47cwxc2id6".to_string(),
        );
        let decoded = TestType::try_decode(encoded);
        assert!(decoded.is_ok());
        assert_eq!(
            decoded.unwrap_or_else(|_| TestType { value: vec![] }).value,
            b"0123456789abcdefghijklmnopqrstuvwxyz"
        );
    }
}
