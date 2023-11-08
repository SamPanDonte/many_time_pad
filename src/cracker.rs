use egui::ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

/// A cracker for a many time pad.
pub struct Cracker {
    combinations: HashMap<u8, HashSet<u8>>,
}

impl Cracker {
    /// Create a new cracker with the given alphabet.
    pub fn new(alphabet: &HashSet<u8>) -> Self {
        let mut combinations = HashMap::new();
        for first in alphabet {
            for second in alphabet {
                let value = *first ^ *second;
                combinations.entry(value).or_insert_with(HashSet::new).insert(*first);
                combinations.entry(value).or_insert_with(HashSet::new).insert(*second);
            }
        }
        Self { combinations }
    }

    /// Crack the given contents with the given key length.
    /// Returns none if the key length is wrong or alphabet is wrong.
    pub fn crack(&self, contents: &[u8], key_length: usize) -> Option<Vec<HashSet<u8>>> {
        let mut remaining_bytes = key_length;
        let mut key: Vec<HashSet<u8>> = {
            let bytes = (0..=255u8).collect();
            vec![bytes; key_length]
        };

        for iteration in 0..(contents.len() / key_length) {
            let chunk = &contents[(iteration * key_length)..((iteration + 1) * key_length)];
            for (index, byte) in chunk.iter().enumerate() {
                let position = index % key_length;

                if key[position].len() == 1 {
                    continue;
                }

                let value = *byte ^ chunk[position];
                let possibilities = self.combinations[&value].iter().map(|value| chunk[position] ^ *value).collect();

                key[position] = key[position].intersection(&possibilities).copied().collect();
                if key[position].len() == 1 {
                    remaining_bytes -= 1;
                } else if key[position].is_empty() {
                    return None;
                }

                if remaining_bytes == 0 {
                    return Some(key);
                }
            }
        }

        Some(key)
    }
}