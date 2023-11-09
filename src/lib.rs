#![warn(clippy::all, rust_2018_idioms)]

use egui::ahash::HashSet;
use encoding::all::WINDOWS_1250;
use encoding::{EncoderTrap, Encoding};

mod cipher;
mod cracker;
mod potential_key;
mod text_encoding;
pub mod ui;

pub use cipher::*;
pub use cracker::*;
pub use potential_key::*;
pub use text_encoding::*;

/// The alphabet used in the challenge. It's Polish letters, numbers, and some punctuation.
const ALPHABET: &str = "aąbcćdeęfghijklłmnńoópqrsśtuvwxyzźżAĄBCĆDEĘFGHIJKLŁMNŃOÓPQRSŚTUVWXYZŹŻ0123456789!\"#%&()*,-.:;?@[] \r\n\u{a0}—’";

pub const WORDS: &str = include_str!("words.txt");

/// Returns a set of bytes that are valid in UTF-8.
pub fn utf8_alphabet() -> HashSet<u8> {
    let mut bytes: HashSet<_> = ALPHABET.as_bytes().iter().cloned().collect();
    bytes.extend(&[169, 166, 171]);
    bytes
}

pub fn utf8_words() -> Vec<Vec<u8>> {
    WORDS
        .split('\n')
        .map(str::as_bytes)
        .map(|x| x.to_vec())
        .collect()
}

/// Returns a set of bytes that are valid in Windows-1250.
pub fn windows1250_alphabet() -> HashSet<u8> {
    WINDOWS_1250
        .encode(ALPHABET, EncoderTrap::Strict)
        .unwrap()
        .into_iter()
        .collect()
}

pub fn windows1250_words() -> Vec<Vec<u8>> {
    WORDS
        .split('\n')
        .map(|x| WINDOWS_1250.encode(x, EncoderTrap::Strict).unwrap())
        .collect()
}
