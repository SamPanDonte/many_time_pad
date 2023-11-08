/// A simple XOR cipher.
pub struct Cipher {
    key: Vec<u8>,
}

impl Cipher {
    /// Create a new cipher with the given key.
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }

    /// Encrypt the given input.
    #[must_use]
    pub fn encrypt(&self, input: &[u8]) -> Vec<u8> {
        let mut output = Vec::with_capacity(input.len());

        for (index, byte) in input.iter().enumerate() {
            output.push(byte ^ self.key[index % self.key.len()]);
        }

        output
    }

    /// Decrypt the given input.
    #[inline]
    #[must_use]
    pub fn decrypt(&self, input: &[u8]) -> Vec<u8> {
        self.encrypt(input)
    }
}