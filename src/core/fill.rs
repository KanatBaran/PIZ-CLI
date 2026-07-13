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
        Self { state: 88172645463325252 }
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