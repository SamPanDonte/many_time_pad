/// Potential key for the cipher.
pub struct PotentialKey {
    pub key: Vec<Vec<u8>>,
    pub positions: Vec<usize>,
}

impl PotentialKey {
    /// Create a new key.
    pub fn new(key: Vec<Vec<u8>>) -> Self {
        Self {
            positions: vec![0; key.len()],
            key,
        }
    }

    /// Get the key.
    pub fn get_current_key(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(self.key.len());
        for (index, value) in self.key.iter().enumerate() {
            key.push(value[self.positions[index]]);
        }
        key
    }
}
