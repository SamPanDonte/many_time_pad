/// Potential key for the cipher.
pub struct PotentialKey {
    key: Vec<Vec<u8>>,
    positions: Vec<usize>,
    uncertain: Vec<bool>,
}

impl PotentialKey {
    /// Create a new key.
    pub fn new(key: Vec<Vec<u8>>) -> Self {
        Self {
            positions: vec![0; key.len()],
            uncertain: vec![true; key.len()],
            key,
        }
    }

    /// Get the key.
    pub fn get_current_key(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(self.key.len());
        for (index, value) in self.key.iter().enumerate() {
            key.push(*value.get(self.positions[index]).unwrap_or(&0));
        }
        key
    }

    /// Is this key position only one possible option.
    pub fn is_decoded(&self, index: usize) -> bool {
        self.key[index].len() == 1
    }

    /// Is value uncertain.
    pub fn is_uncertain(&self, index: usize) -> bool {
        self.uncertain[index]
    }

    /// Set value for this key position.
    pub fn set_value(&mut self, index: usize, value: u8) {
        self.uncertain[index] = false;
        self.positions[index] = self.key[index].iter().position(|&r| r == value).unwrap();
    }

    /// Is value possible for this key position.
    pub fn is_possible(&self, index: usize, value: u8) -> bool {
        self.key[index].contains(&value)
    }

    /// Is value not possible to decode.
    pub fn is_error(&self, index: usize) -> bool {
        self.key[index].is_empty()
    }

    /// Get all possible values for this key position.
    pub fn get_possibilities(&self, index: usize) -> &Vec<u8> {
        &self.key[index]
    }

    /// Sets value for key index.
    pub fn accept_value(&mut self, index: usize) {
        let value = self.key[index][self.positions[index]];
        self.key[index].clear();
        self.key[index].push(value);
        self.positions[index] = 0;
    }
}
