use egui::ahash::HashSet;
use encoding::all::WINDOWS_1250;
use encoding::{DecoderTrap, Encoding};
use std::str;

#[cfg(not(target_arch = "wasm32"))]
use clap::ValueEnum;

/// Text encoding.
#[cfg(not(target_arch = "wasm32"))]
#[derive(PartialEq, ValueEnum, Clone, Debug)]
pub enum TextEncoding {
    WINDOWS1250,
    UTF8,
}

#[cfg(target_arch = "wasm32")]
#[derive(PartialEq, Clone, Debug)]
pub enum TextEncoding {
    WINDOWS1250,
    UTF8,
}

impl TextEncoding {
    /// Decode bytes to string.
    pub fn decode(&self, bytes: &[u8]) -> Option<String> {
        match self {
            TextEncoding::WINDOWS1250 => WINDOWS_1250.decode(bytes, DecoderTrap::Strict).ok(),
            TextEncoding::UTF8 => str::from_utf8(bytes).ok().map(ToString::to_string),
        }
    }

    /// Get alphabet.
    pub fn alphabet(&self) -> HashSet<u8> {
        match self {
            TextEncoding::WINDOWS1250 => crate::windows1250_alphabet(),
            TextEncoding::UTF8 => crate::utf8_alphabet(),
        }
    }

    /// Get most popular words.
    pub fn words(&self) -> Vec<Vec<u8>> {
        match self {
            TextEncoding::WINDOWS1250 => crate::windows1250_words(),
            TextEncoding::UTF8 => crate::utf8_words(),
        }
    }
}

impl Default for TextEncoding {
    fn default() -> Self {
        Self::WINDOWS1250
    }
}

impl std::fmt::Display for TextEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextEncoding::WINDOWS1250 => write!(f, "Windows-1250"),
            TextEncoding::UTF8 => write!(f, "UTF-8"),
        }
    }
}
