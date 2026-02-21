//! Encoding and decoding utilities for representing data in compact textual forms.
//!
//! This crate provides conversion between raw bytes and encoded strings.
//!
//! See [`ByteVec`], [`EncodedString`], and [`Encoding`] for the main entry points.
#![deny(missing_docs)]

//! Types and algorithms for encoding/decoding data.

/// Serialization algorithms and implementations.
pub mod algorithm;

/// Encoder trait for encoding and decoding data.
pub mod encoder;

/// Raw byte representation of serializable data.
pub mod byte_vec;

/// String representation of serialized data.
pub mod encoded_string;

/// Error type for serialization operations.
pub mod serialise_error;

/// Supported serialization formats.
pub mod encoding;

pub use algorithm::base36::Base36;
pub use algorithm::base58::Base58;
pub use algorithm::base64::Base64;
pub use algorithm::hex::Hex;
pub use algorithm::uuencode::Uuencode;
pub use byte_vec::ByteVec;
pub use encoded_string::EncodedString;
pub use encoder::Encoder;
pub use encoding::Encoding;
pub use serialise_error::SerialiseError;
