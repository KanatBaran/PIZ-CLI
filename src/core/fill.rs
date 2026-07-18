/* Pi */
pub struct PiGenerator {
    digits: &'static [u8],
    index: usize,
}

impl PiGenerator {
    pub fn new() -> Self {
        Self {
            digits: b"31415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679",
            index: 0,
        }
    }
}

// PiGenerator is now a structure that generates data sequentially.
impl Iterator for PiGenerator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.digits[self.index];
        self.index = (self.index + 1) % self.digits.len();
        Some(val)
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

    // TODO: Add unit tests for PiGenerator once it is updated to generate actual Pi digits.
    // Currently, PiGenerator does not produce true Pi values and is planned to be rewritten.

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
