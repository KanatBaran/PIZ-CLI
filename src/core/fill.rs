/* Pi */
const PI_SEED: &[u8] = b"31415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
const BUFFER_SIZE: usize = 64 * 1024;

pub struct PiGenerator {
    reader: blake3::OutputReader, // object OutputReader
    buffer: [u8; BUFFER_SIZE],
    index: usize,
}

impl PiGenerator {
    pub fn new() -> PiGenerator {
        // Create hasher and add seed
        let mut hasher = blake3::Hasher::new();
        hasher.update(PI_SEED);

        let mut reader = hasher.finalize_xof(); // It generates infinite hash data.
        let mut buffer = [0u8; BUFFER_SIZE];

        // It populates the buffer with the specified hash data.
        reader.fill(&mut buffer);

        PiGenerator {
            reader,
            buffer,
            index: 0,
        }
    }
}

// PiGenerator is now a structure that generates data sequentially using BLAKE3 XOF.
impl Iterator for PiGenerator {
    type Item = u8;

    // The function that returns the next byte.
    fn next(&mut self) -> Option<Self::Item> {
        // If my index exceeds the buffer size, the buffer is full.
        if self.index >= self.buffer.len() {
            self.reader.fill(&mut self.buffer);
            self.index = 0;
        }

        // Returns the next data in the buffer.
        let byte = self.buffer[self.index];
        self.index += 1;
        Some(byte)
    }
}
/* ./Pi */

/* Random */
pub struct RandomGenerator {
    state: u64,
}

impl RandomGenerator {
    pub fn new() -> Self {
        // seed
        Self {
            state: 88172645463325252,
        }
    }
}

impl Iterator for RandomGenerator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        // Xorshift pseudo-random
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        Some((self.state & 0xFF) as u8)
    }
}
/* ./Random */

/* Unit Tests */
#[cfg(test)]
mod tests {
    use super::*;

    // --------------------
    // PiGenerator
    // --------------------

    // Verifies that PiGenerator is deterministic and always generates the exact same byte stream.
    #[test]
    fn test_pi_generator_deterministic() {
        let mut generator1 = PiGenerator::new();
        let mut generator2 = PiGenerator::new();

        let seq1: Vec<u8> = (&mut generator1).take(100).collect();
        let seq2: Vec<u8> = (&mut generator2).take(100).collect();

        assert_eq!(seq1, seq2);
    }

    // Verifies that PiGenerator correctly refills internal buffer when crossing buffer boundaries.
    #[test]
    fn test_pi_generator_across_buffer_boundary() {
        let mut generator1 = PiGenerator::new();
        let mut generator2 = PiGenerator::new();

        // Consume past the 64KB (65536 bytes) buffer boundary
        let seq1: Vec<u8> = (&mut generator1).take(70000).collect();
        let seq2: Vec<u8> = (&mut generator2).take(70000).collect();

        assert_eq!(seq1.len(), 70000);
        assert_eq!(seq1, seq2);
    }

    // --------------------
    // RandomGenerator
    // --------------------

    // Verifies that RandomGenerator is deterministic and always generates the exact same sequence for a given seed.
    #[test]
    fn test_random_generator_deterministic() {
        let mut generator1 = RandomGenerator::new();
        let mut generator2 = RandomGenerator::new();

        let seq1: Vec<u8> = (&mut generator1).take(20).collect();
        let seq2: Vec<u8> = (&mut generator2).take(20).collect();

        assert_eq!(seq1, seq2);
    }

    // Verifies that RandomGenerator does not yield a constant value and changes state on each step.
    #[test]
    fn test_random_generator_non_constant() {
        let mut generator = RandomGenerator::new();
        let seq: Vec<u8> = (&mut generator).take(100).collect();

        // Check that they are not all the same byte
        let first_byte = seq[0];
        let all_same = seq.iter().all(|&b| b == first_byte);
        assert!(
            !all_same,
            "RandomGenerator produced a constant sequence of bytes"
        );
    }
}
/* ./Unit Tests */
