# base_xx

[![Crates.io](https://img.shields.io/crates/v/base_xx.svg)](https://crates.io/crates/base_xx)
[![Documentation](https://docs.rs/base_xx/badge.svg)](https://docs.rs/base_xx)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for encoding and decoding data in various base-X formats.

## Features

- Base36 encoding (0-9 and a-z)
- Base58 encoding (Bitcoin-style)
- Trait-based design for extensibility
- Zero-copy where possible
- Comprehensive error handling
- No unsafe code

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
base_xx = "0.5.0"
```

## Usage

### Basic Encoding

```rust
use base_xx::{ByteVec, Encoding};

let bytes = ByteVec::new(b"Hello, world!".to_vec());
let base36 = bytes.clone().try_encode(Encoding::Base36)?;
let base58 = bytes.try_encode(Encoding::Base58)?;

println!("Base36: {}", base36);
println!("Base58: {}", base58);
```

### Implementing for Custom Types

```rust
use base_xx::{ByteVec, Encodable, Encoding, SerialiseError};

struct MyType {
    data: Vec<u8>,
}

impl TryFrom<&MyType> for ByteVec {
    type Error = SerialiseError;

    fn try_from(value: &MyType) -> Result<Self, Self::Error> {
        Ok(ByteVec::new(value.data.clone()))
    }
}

impl Encodable for MyType {}

// Now you can encode MyType
let my_data = MyType { data: vec![1, 2, 3] };
let encoded = my_data.try_encode(Encoding::Base36)?;
```

### Decoding

```rust
use base_xx::{ByteVec, Decodable, EncodedString, Encoding, SerialiseError};

// Implement TryFrom<ByteVec> for your type
impl TryFrom<ByteVec> for MyType {
    type Error = SerialiseError;

    fn try_from(value: ByteVec) -> Result<Self, Self::Error> {
        Ok(MyType {
            data: value.get_bytes().to_vec()
        })
    }
}

// Then implement Decodable
impl Decodable for MyType {}

// Now you can decode
let encoded = EncodedString::new(Encoding::Base36, "...".to_string());
let decoded = MyType::try_decode(encoded)?;
```

## Supported Encodings

- **Base36**: Uses digits 0-9 and lowercase letters a-z. Good for case-insensitive human-readable output.
- **Base58**: Uses Bitcoin-style alphabet, omitting similar-looking characters. Ideal for user-facing identifiers.

## Error Handling

All encoding/decoding operations return `Result<T, SerialiseError>`. The `SerialiseError` type provides detailed error information for:

- Invalid characters in input
- Unsupported encoding formats
- Size constraint violations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Robert Forster (<robert.forster@beamvex.com>)
