use crate::{Base36, EncodedString, Encoder, Encoding, SerialiseError};

/// Raw byte representation of serializable data.
///
/// This type represents the raw bytes of a serializable structure along with
/// its type information. It serves as an intermediate format between the
/// original data and its string representation.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Implements encoding functionality for a type that can be converted to bytes.
///
/// This trait generates implementation for encoding methods that allow converting
/// the implementing type into various string representations (e.g., Base36, Base58).
///
pub trait Encodable
where
    for<'a> ByteVec: TryFrom<&'a Self, Error = SerialiseError>,
{
    /// Encodes this type using the specified `Encoding`.
    ///
    /// # Parameters
    /// * `value` - The value to encode
    /// * `encoding` - The encoding to use when encoding this type.
    ///
    /// # Returns
    /// A `Result` containing the encoded string if successful, or a `SerialiseError` if an error occurs.
    ///
    /// # Errors
    /// * `SerialiseError` - If the specified encoding is unsupported or an error occurs during serialisation.
    #[must_use = "The result of this function is a `Result` containing the encoded string if successful, or a `SerialiseError` if an error occurs."]
    fn try_encode(&self, encoding: Encoding) -> Result<EncodedString, SerialiseError> {
        match ByteVec::try_from(self) {
            Ok(bytes) => match encoding {
                Encoding::Base36 => match Base36::try_encode(bytes.get_bytes()) {
                    Ok(encoded) => Ok(EncodedString::new(Encoding::Base36, encoded)),
                    Err(error) => Err(error),
                },
                _ => Err(SerialiseError::new("Unsupported encoding".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encodable_encoding_base36() {
        struct Test {
            bytes: Vec<u8>,
        }

        impl TryFrom<&Test> for ByteVec {
            type Error = SerialiseError;
            fn try_from(value: &Test) -> Result<Self, SerialiseError> {
                Ok(Self::new(value.bytes.clone()))
            }
        }

        impl Encodable for Test {}

        let test = Test {
            bytes: b"0123456789abcdefghijklmnopqrstuvwxyz".to_vec(),
        };

        let encoded = test.try_encode(Encoding::Base36);
        assert!(encoded.is_ok());
        assert_eq!(
            encoded
                .unwrap_or_else(|_| EncodedString::new(Encoding::Base36, "no match".to_string()))
                .get_string(),
            "2dbg0rhouyms2hsh4jiluolq0rx1et8yty277nr9mwq20b47cwxc2id6"
        );
    }
}
