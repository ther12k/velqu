//! Text encoding and decoding primitives according to the Encoding Standard (M27-006-A).
//!
//! Provides bounded, conformant UTF-8 TextEncoder and TextDecoder implementations.

use std::fmt;

/// Maximum buffer size in bytes for a single text encode or decode operation (16 MB).
pub const MAX_TEXT_BUFFER_LEN: usize = 16 * 1024 * 1024;

/// Typed text encoding errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextEncodingError {
    BufferTooLarge { len: usize, max: usize },
    InvalidEncoding(String),
    FatalDecodeError { offset: usize },
}

impl fmt::Display for TextEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextEncodingError::BufferTooLarge { len, max } => {
                write!(
                    f,
                    "text buffer length {len} exceeds maximum allowed limit {max}"
                )
            }
            TextEncodingError::InvalidEncoding(label) => {
                write!(f, "unsupported or invalid encoding label: {label:?}")
            }
            TextEncodingError::FatalDecodeError { offset } => {
                write!(f, "fatal decode error at byte offset {offset}")
            }
        }
    }
}

impl std::error::Error for TextEncodingError {}

/// Core UTF-8 TextEncoder primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextEncoderModel;

impl TextEncoderModel {
    pub fn encoding() -> &'static str {
        "utf-8"
    }

    /// Encode a UTF-8 string to bytes with bounded buffer checks.
    pub fn encode(input: &str) -> Result<Vec<u8>, TextEncodingError> {
        let bytes = input.as_bytes();
        if bytes.len() > MAX_TEXT_BUFFER_LEN {
            return Err(TextEncodingError::BufferTooLarge {
                len: bytes.len(),
                max: MAX_TEXT_BUFFER_LEN,
            });
        }
        Ok(bytes.to_vec())
    }

    /// Encode into a pre-allocated destination buffer (encodeInto semantics).
    /// Returns (read_chars, written_bytes).
    pub fn encode_into(source: &str, destination: &mut [u8]) -> (usize, usize) {
        let mut read = 0;
        let mut written = 0;
        for c in source.chars() {
            let len = c.len_utf8();
            if written + len > destination.len() {
                break;
            }
            c.encode_utf8(&mut destination[written..written + len]);
            written += len;
            read += 1;
        }
        (read, written)
    }
}

/// Options for TextDecoder configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextDecoderOptions {
    pub fatal: bool,
    pub ignore_bom: bool,
}

/// Core UTF-8 TextDecoder primitive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextDecoderModel {
    pub encoding: String,
    pub options: TextDecoderOptions,
}

impl TextDecoderModel {
    /// Create a new UTF-8 decoder. Only "utf-8" and standard aliases are supported.
    pub fn new(
        label: Option<&str>,
        options: TextDecoderOptions,
    ) -> Result<Self, TextEncodingError> {
        let norm = match label {
            None => "utf-8",
            Some(l) => match l.trim().to_ascii_lowercase().as_str() {
                "utf-8" | "utf8" | "unicode-1-1-utf-8" => "utf-8",
                other => return Err(TextEncodingError::InvalidEncoding(other.to_string())),
            },
        };
        Ok(TextDecoderModel {
            encoding: norm.to_string(),
            options,
        })
    }

    /// Decode raw bytes to a String.
    pub fn decode(&self, input: &[u8]) -> Result<String, TextEncodingError> {
        if input.len() > MAX_TEXT_BUFFER_LEN {
            return Err(TextEncodingError::BufferTooLarge {
                len: input.len(),
                max: MAX_TEXT_BUFFER_LEN,
            });
        }

        // Handle BOM (U+FEFF) if not ignored
        let data = if !self.options.ignore_bom && input.starts_with(&[0xEF, 0xBB, 0xBF]) {
            &input[3..]
        } else {
            input
        };

        if self.options.fatal {
            match std::str::from_utf8(data) {
                Ok(s) => Ok(s.to_string()),
                Err(e) => Err(TextEncodingError::FatalDecodeError {
                    offset: e.valid_up_to(),
                }),
            }
        } else {
            Ok(String::from_utf8_lossy(data).into_owned())
        }
    }
}

impl Default for TextDecoderModel {
    fn default() -> Self {
        Self::new(None, TextDecoderOptions::default()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_encoder_encodes_utf8_losslessly() {
        let input = "Hello, 🌍 Velqu! ⚡";
        let bytes = TextEncoderModel::encode(input).unwrap();
        assert_eq!(bytes, input.as_bytes());
        assert_eq!(TextEncoderModel::encoding(), "utf-8");
    }

    #[test]
    fn text_encoder_encode_into() {
        let input = "Hello 🌍";
        let mut dest = [0u8; 8];
        let (read, written) = TextEncoderModel::encode_into(input, &mut dest);
        assert_eq!(read, 6); // "Hello " = 6 chars (6 bytes). 🌍 needs 4 bytes (6+4=10 > 8) so it stops
        assert_eq!(written, 6);
        assert_eq!(&dest[..6], b"Hello ");
    }

    #[test]
    fn text_decoder_utf8_baseline() {
        let decoder = TextDecoderModel::default();
        let input = "Hello, 🌍 Velqu! ⚡".as_bytes();
        assert_eq!(decoder.decode(input).unwrap(), "Hello, 🌍 Velqu! ⚡");
    }

    #[test]
    fn text_decoder_bom_handling() {
        let with_bom = [0xEF, 0xBB, 0xBF, b'h', b'i'];
        let normal_decoder = TextDecoderModel::default();
        assert_eq!(normal_decoder.decode(&with_bom).unwrap(), "hi");

        let ignore_bom_decoder = TextDecoderModel::new(
            Some("utf-8"),
            TextDecoderOptions {
                fatal: false,
                ignore_bom: true,
            },
        )
        .unwrap();
        assert_eq!(ignore_bom_decoder.decode(&with_bom).unwrap(), "\u{FEFF}hi");
    }

    #[test]
    fn text_decoder_fatal_mode_rejects_invalid_utf8() {
        let invalid_utf8 = [0xff, 0xfe, 0xfd];
        let fatal_decoder = TextDecoderModel::new(
            Some("utf-8"),
            TextDecoderOptions {
                fatal: true,
                ignore_bom: false,
            },
        )
        .unwrap();
        assert!(fatal_decoder.decode(&invalid_utf8).is_err());

        let non_fatal = TextDecoderModel::default();
        let decoded = non_fatal.decode(&invalid_utf8).unwrap();
        assert!(decoded.contains('\u{FFFD}'));
    }

    #[test]
    fn buffer_size_limit_is_enforced() {
        let huge = vec![b'a'; MAX_TEXT_BUFFER_LEN + 10];
        let decoder = TextDecoderModel::default();
        assert_eq!(
            decoder.decode(&huge),
            Err(TextEncodingError::BufferTooLarge {
                len: MAX_TEXT_BUFFER_LEN + 10,
                max: MAX_TEXT_BUFFER_LEN,
            })
        );
    }

    #[test]
    fn unsupported_encoding_label_fails_closed() {
        assert!(TextDecoderModel::new(Some("iso-8859-1"), TextDecoderOptions::default()).is_err());
        assert!(
            TextDecoderModel::new(Some("windows-1252"), TextDecoderOptions::default()).is_err()
        );
        assert!(TextDecoderModel::new(Some("utf-16"), TextDecoderOptions::default()).is_err());
    }
}
