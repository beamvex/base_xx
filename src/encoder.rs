use crate::SerialiseError;

/// Trait for types that can encode and decode data
pub trait Encoder {
    /// Attempts to encode bytes into a string representation
    ///
    /// # Arguments
    /// * `bytes` - The bytes to encode
    ///
    /// # Returns
    /// The encoded string representation
    ///
    /// # Errors
    /// Returns `SerialiseError` if encoding fails
    fn try_encode(bytes: &[u8]) -> Result<String, SerialiseError>;

    /// Attempts to decode a string back into bytes
    ///
    /// # Arguments
    /// * `encoded` - The string to decode
    ///
    /// # Returns
    /// The decoded bytes
    ///
    /// # Errors
    /// Returns `SerialiseError` if decoding fails
    fn try_decode(encoded: &str) -> Result<Vec<u8>, SerialiseError>;
}
