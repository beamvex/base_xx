use crate::serialise::Encoding;

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
