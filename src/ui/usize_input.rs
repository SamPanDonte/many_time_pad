use egui::TextBuffer;
use std::fmt::Display;
use std::num::NonZeroUsize;
use std::ops::{Deref, Range};

pub struct NonZeroUsizeInput(NonZeroUsize, String);

impl NonZeroUsizeInput {
    pub fn new(value: NonZeroUsize) -> Self {
        Self(value, value.to_string())
    }
}

impl From<NonZeroUsize> for NonZeroUsizeInput {
    fn from(value: NonZeroUsize) -> Self {
        Self::new(value)
    }
}

impl Display for NonZeroUsizeInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.1.fmt(f)
    }
}

impl Deref for NonZeroUsizeInput {
    type Target = NonZeroUsize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TextBuffer for NonZeroUsizeInput {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.1.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let mut copy = self.1.clone();
        copy.insert_text(text, char_index);
        if let Ok(value) = copy.parse() {
            self.0 = value;
            self.1.insert_text(text, char_index)
        } else {
            0
        }
    }

    fn delete_char_range(&mut self, char_range: Range<usize>) {
        self.1.delete_char_range(char_range);
        if let Ok(value) = self.1.parse() {
            self.0 = value;
        }
    }
}
