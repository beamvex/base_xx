//! Types and algorithms for encoding/decoding data.

/// Serialization algorithms and implementations.
pub mod algorithm;

/// Raw byte representation of serializable data.
pub mod byte_vec;

/// String representation of serialized data.
pub mod encoded_string;

/// Error type for serialization operations.
pub mod serialise_error;

/// Supported serialization formats.
pub mod encoding;

pub use algorithm::base36::Base36;
//pub use algorithm::base58::Base58;
//pub use algorithm::base64::Base64;
//pub use algorithm::hex::Hex;
//pub use algorithm::uuencode::Uuencode;
pub use byte_vec::ByteVec;
pub use encoded_string::EncodedString;
pub use encoding::Encoding;
pub use serialise_error::SerialiseError;
