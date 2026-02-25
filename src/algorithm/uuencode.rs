use crate::{EncodedString, Encoder, Encoding, SerialiseError};

/// `uuencode` implementation.
#[derive(Debug)]
pub struct Uuencode {}

impl Uuencode {
    const fn enc6(v: u8) -> u8 {
        let v = v & 0x3f;
        if v == 0 { b'`' } else { v + 0x20 }
    }

    const fn dec6(c: u8) -> Option<u8> {
        match c {
            b'`' | b' ' => Some(0),
            0x20..=0x5f => Some((c - 0x20) & 0x3f),
            _ => None,
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    fn enc_len(n: usize) -> u8 {
        u8::try_from(n).map_or_else(|_| Self::enc6(0), Self::enc6)
    }

    #[allow(clippy::missing_const_for_fn)]
    fn dec_len(c: u8) -> Option<usize> {
        Self::dec6(c).map(usize::from)
    }

    /// Uuencode bytes using the traditional uuencode line format (45 bytes per line).
    ///
    /// Output has one or more lines. Each line begins with an encoded length character,
    /// followed by encoded data, and ends with `\n`. The final line is "\`\n".
    #[must_use = "this returns the uuencoded string but does nothing if unused"]
    pub fn to_uuencode(bytes: &[u8]) -> String {
        let mut out: Vec<u8> = Vec::new();

        for chunk in bytes.chunks(45) {
            out.push(Self::enc_len(chunk.len()));

            for triple in chunk.chunks(3) {
                let b0 = triple[0];
                let b1 = *triple.get(1).unwrap_or(&0);
                let b2 = *triple.get(2).unwrap_or(&0);

                let c0 = (b0 >> 2) & 0x3f;
                let c1 = ((b0 << 4) | (b1 >> 4)) & 0x3f;
                let c2 = ((b1 << 2) | (b2 >> 6)) & 0x3f;
                let c3 = b2 & 0x3f;

                out.push(Self::enc6(c0));
                out.push(Self::enc6(c1));
                out.push(Self::enc6(c2));
                out.push(Self::enc6(c3));
            }

            out.push(b'\n');
        }

        out.push(b'`');
        out.push(b'\n');

        let mut s = String::with_capacity(out.len());
        for b in out {
            s.push(b as char);
        }
        s
    }

    /// Decode a uuencoded string (traditional uuencode line format) into bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if `uuencoded` contains invalid uuencode characters or malformed lines.
    pub fn from_uuencode(uuencoded: &str) -> Result<Vec<u8>, SerialiseError> {
        let mut out: Vec<u8> = Vec::new();

        for line in uuencoded.lines() {
            if line.is_empty() {
                continue;
            }

            let mut it = line.as_bytes().iter().copied();
            let len_ch = it.next().ok_or_else(|| {
                SerialiseError::new("uuencode line must have a length character".to_string())
            })?;
            let line_len = Self::dec_len(len_ch).ok_or_else(|| {
                SerialiseError::new("invalid uuencode length character".to_string())
            })?;
            if line_len == 0 {
                break;
            }

            let mut produced = 0usize;
            while produced < line_len {
                let a = it
                    .next()
                    .ok_or_else(|| SerialiseError::new("truncated uuencode data".to_string()))?;
                let b = it
                    .next()
                    .ok_or_else(|| SerialiseError::new("truncated uuencode data".to_string()))?;
                let c = it
                    .next()
                    .ok_or_else(|| SerialiseError::new("truncated uuencode data".to_string()))?;
                let d = it
                    .next()
                    .ok_or_else(|| SerialiseError::new("truncated uuencode data".to_string()))?;

                let a = Self::dec6(a)
                    .ok_or_else(|| SerialiseError::new("invalid uuencode character".to_string()))?;
                let b = Self::dec6(b)
                    .ok_or_else(|| SerialiseError::new("invalid uuencode character".to_string()))?;
                let c = Self::dec6(c)
                    .ok_or_else(|| SerialiseError::new("invalid uuencode character".to_string()))?;
                let d = Self::dec6(d)
                    .ok_or_else(|| SerialiseError::new("invalid uuencode character".to_string()))?;

                let o0 = (a << 2) | (b >> 4);
                let o1 = (b << 4) | (c >> 2);
                let o2 = (c << 6) | d;

                for o in [o0, o1, o2] {
                    if produced < line_len {
                        out.push(o);
                        produced += 1;
                    }
                }
            }
        }

        Ok(out)
    }
}

impl Encoder for Uuencode {
    fn try_encode(bytes: &[u8]) -> Result<EncodedString, SerialiseError> {
        Ok(EncodedString::new(
            Encoding::Uuencode,
            Self::to_uuencode(bytes),
        ))
    }

    fn try_decode(encoded: &EncodedString) -> Result<Vec<u8>, SerialiseError> {
        Self::from_uuencode(encoded.get_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_uuencode() {
        let bytes = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let uuencode = Uuencode::to_uuencode(bytes);
        assert_eq!(
            uuencode,
            "D,#$R,S0U-C<X.6%B8V1E9F=H:6IK;&UN;W!Q<G-T=79W>'EZ\n`\n"
        );
    }

    #[test]
    fn test_from_uuencode() {
        let string = "D,#$R,S0U-C<X.6%B8V1E9F=H:6IK;&UN;W!Q<G-T=79W>'EZ\n`\n";
        assert!(matches!(
            Uuencode::from_uuencode(string),
            Ok(bytes) if bytes == b"0123456789abcdefghijklmnopqrstuvwxyz"
        ));
    }

    #[test]
    fn test_from_invalid_uuencode_is_err() {
        let string = "gg";
        assert!(Uuencode::from_uuencode(string).is_err());
    }
}
