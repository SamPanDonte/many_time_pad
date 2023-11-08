#![warn(clippy::all, rust_2018_idioms)]

use egui::ahash::HashSet;
use encoding::all::WINDOWS_1250;
use encoding::{EncoderTrap, Encoding};

pub mod cipher;
pub mod cracker;
pub mod ui;

/// The alphabet used in the challenge. It's Polish letters, numbers, and some punctuation.
const ALPHABET: &str = "aąbcćdeęfghijklłmnńoópqrsśtuvwxyzźżAĄBCĆDEĘFGHIJKLŁMNŃOÓPQRSŚTUVWXYZŹŻ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \r\n\t\u{a0}—";

/// Returns a set of bytes that are valid in UTF-8.
pub fn utf8_alphabet() -> HashSet<u8> {
    let mut bytes: HashSet<_> = ALPHABET.as_bytes().iter().cloned().collect();
    bytes.extend(&[169, 166, 171]);
    bytes
}

/// Returns a set of bytes that are valid in Windows-1250.
pub fn windows1250_alphabet() -> HashSet<u8> {
    WINDOWS_1250
        .encode(ALPHABET, EncoderTrap::Strict)
        .unwrap()
        .into_iter()
        .collect()
}
