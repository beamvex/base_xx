use crate::{
    Base36, ByteVec, Encoder, Encoding, SerialiseError,
    algorithm::{Base58, Base64, Hex, Uuencode},
};

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
pub trait Decodable
where
    Self: TryFrom<ByteVec, Error = SerialiseError>,
{
    /// Attempts to decode an encoded string into this type.
    ///
    /// # Errors
    /// Returns `Err` if the underlying decoding fails.
    #[must_use = "decoding returns a result that must be handled"]
    fn try_decode(encoded_string: EncodedString) -> Result<Self, SerialiseError>
    where
        Self: Sized,
    {
        match encoded_string.get_encoding() {
            Encoding::Base36 => match Base36::try_decode(encoded_string.get_string()) {
                Ok(bytes) => Self::try_from(ByteVec::new(bytes)),
                Err(e) => Err(SerialiseError::new(e.to_string())),
            },
            Encoding::Base58 => match Base58::try_decode(encoded_string.get_string()) {
                Ok(bytes) => Self::try_from(ByteVec::new(bytes)),
                Err(e) => Err(SerialiseError::new(e.to_string())),
            },
            Encoding::Base64 => match Base64::try_decode(encoded_string.get_string()) {
                Ok(bytes) => Self::try_from(ByteVec::new(bytes)),
                Err(e) => Err(SerialiseError::new(e.to_string())),
            },
            Encoding::Hex => match Hex::try_decode(encoded_string.get_string()) {
                Ok(bytes) => Self::try_from(ByteVec::new(bytes)),
                Err(e) => Err(SerialiseError::new(e.to_string())),
            },
            Encoding::Uuencode => match Uuencode::try_decode(encoded_string.get_string()) {
                Ok(bytes) => Self::try_from(ByteVec::new(bytes)),
                Err(e) => Err(SerialiseError::new(e.to_string())),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                    value: value.get_bytes().to_vec(),
                })
            }
        }

        impl Decodable for TestType {}

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

    #[test]
    fn test_decoable_decode_base58() {
        #[derive(Debug, PartialEq)]
        struct TestType {
            value: Vec<u8>,
        }

        impl TryFrom<ByteVec> for TestType {
            type Error = SerialiseError;

            fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
                Ok(Self {
                    value: value.get_bytes().to_vec(),
                })
            }
        }

        impl Decodable for TestType {}

        let encoded = EncodedString::new(
            Encoding::Base58,
            "NE1FfXYqCHge2p4MZ56o8gdrDWMiHXPJLXk9ixxKgUebU7VqB".to_string(),
        );
        let decoded = TestType::try_decode(encoded);
        assert!(decoded.is_ok());
        assert_eq!(
            decoded.unwrap_or_else(|_| TestType { value: vec![] }).value,
            b"0123456789abcdefghijklmnopqrstuvwxyz"
        );
    }

    #[test]
    fn test_decoable_decode_base64() {
        #[derive(Debug, PartialEq)]
        struct TestType {
            value: Vec<u8>,
        }

        impl TryFrom<ByteVec> for TestType {
            type Error = SerialiseError;

            fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
                Ok(Self {
                    value: value.get_bytes().to_vec(),
                })
            }
        }

        impl Decodable for TestType {}

        let encoded = EncodedString::new(
            Encoding::Base64,
            "MDEyMzQ1Njc4OWFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6".to_string(),
        );
        let decoded = TestType::try_decode(encoded);
        assert!(decoded.is_ok());
        assert_eq!(
            decoded.unwrap_or_else(|_| TestType { value: vec![] }).value,
            b"0123456789abcdefghijklmnopqrstuvwxyz"
        );
    }

    #[test]
    fn test_decoable_decode_hex() {
        #[derive(Debug, PartialEq)]
        struct TestType {
            value: Vec<u8>,
        }

        impl TryFrom<ByteVec> for TestType {
            type Error = SerialiseError;

            fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
                Ok(Self {
                    value: value.get_bytes().to_vec(),
                })
            }
        }

        impl Decodable for TestType {}

        let encoded = EncodedString::new(
            Encoding::Hex,
            "303132333435363738396162636465666768696a6b6c6d6e6f70".to_string(),
        );
        let decoded = TestType::try_decode(encoded);
        assert!(decoded.is_ok());
        assert_eq!(
            decoded.unwrap_or_else(|_| TestType { value: vec![] }).value,
            b"0123456789abcdefghijklmnop"
        );
    }

    #[test]
    fn test_decoable_decode_uuencode() {
        #[derive(Debug, PartialEq)]
        struct TestType {
            value: Vec<u8>,
        }

        impl TryFrom<ByteVec> for TestType {
            type Error = SerialiseError;

            fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
                Ok(Self {
                    value: value.get_bytes().to_vec(),
                })
            }
        }

        impl Decodable for TestType {}

        let encoded = EncodedString::new(
            Encoding::Uuencode,
            "D,#$R,S0U-C<X.6%B8V1E9F=H:6IK;&UN;W!Q<G-T=79W>'EZ\n`\n".to_string(),
        );
        let decoded = TestType::try_decode(encoded);
        assert!(decoded.is_ok());
        assert_eq!(
            decoded.unwrap_or_else(|_| TestType { value: vec![] }).value,
            b"0123456789abcdefghijklmnopqrstuvwxyz"
        );
    }
}
