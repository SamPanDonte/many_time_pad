use crate::{PotentialKey, TextEncoding};
use egui::ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use std::num::NonZeroUsize;

/// A cracker for a many time pad.
pub struct Cracker {
    combinations: HashMap<u8, HashSet<u8>>,
    words: Vec<Vec<u8>>,
}

impl Cracker {
    /// Create a new cracker with the given alphabet.
    pub fn new(encoding: &TextEncoding) -> Self {
        let alphabet = encoding.alphabet();
        let words = encoding.words();

        let mut combinations = HashMap::new();
        for first in &alphabet {
            for second in &alphabet {
                let value = *first ^ *second;
                combinations
                    .entry(value)
                    .or_insert_with(HashSet::new)
                    .insert(*first);
                combinations
                    .entry(value)
                    .or_insert_with(HashSet::new)
                    .insert(*second);
            }
        }
        Self {
            combinations,
            words,
        }
    }

    /// Crack the given contents with the given key length.
    /// Returns none if the key length is wrong or alphabet is wrong.
    pub fn crack(&self, contents: &[u8], key_length: NonZeroUsize) -> PotentialKey {
        let potential_key = self.xor_attack(contents, key_length);
        let potential_key: Vec<Vec<u8>> = potential_key
            .into_iter()
            .map(|key| key.into_iter().collect())
            .collect();
        let mut potential_key = PotentialKey::new(potential_key);

        for word in &self.words {
            for index in 0..(contents.len() - word.len()) {
                let mut possible = true;

                for (i, byte) in word.iter().zip(&contents[index..]).enumerate() {
                    if !potential_key.is_possible((index + i) % key_length, *byte.0 ^ *byte.1) {
                        possible = false;
                        break;
                    }
                }

                if !possible {
                    continue;
                }

                for (i, byte) in word.iter().zip(&contents[index..]).enumerate() {
                    potential_key.set_value((index + i) % key_length, *byte.0 ^ *byte.1);
                }
            }
        }

        potential_key
    }

    fn xor_attack(&self, contents: &[u8], key_length: NonZeroUsize) -> Vec<HashSet<u8>> {
        let key_length = key_length.get();
        let mut remaining_bytes = key_length;
        let mut key: Vec<HashSet<u8>> = {
            let bytes = (0..=255u8).collect();
            vec![bytes; key_length]
        };

        for iteration in 0..(contents.len() / key_length) {
            let chunk = &contents[(iteration * key_length)..((iteration + 1) * key_length)];
            for (index, byte) in chunk.iter().enumerate() {
                let position = index % key_length;

                if key[position].len() <= 1 {
                    continue;
                }

                let value = *byte ^ chunk[position];
                let possibilities = self.combinations[&value]
                    .iter()
                    .map(|value| chunk[position] ^ *value)
                    .collect();

                key[position] = key[position]
                    .intersection(&possibilities)
                    .copied()
                    .collect();
                if key[position].len() <= 1 {
                    remaining_bytes -= 1;
                }

                if remaining_bytes == 0 {
                    return key;
                }
            }
        }
        key
    }
}
