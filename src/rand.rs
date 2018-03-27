//! This module provies PRNG to generate
//! a string of random u8s
use std::num::{Wrapping};
use std::time::{SystemTime, UNIX_EPOCH};
/// A struct for storing the required elements to perform
/// the [Middle Square Weyl Sequence](https://en.wikipedia.org/wiki/Middle-square_method#Middle_Square_Weyl_Sequence_PRNG)
pub struct Rng {
    seed: Wrapping<u8>,
    w: Wrapping<u8>,
    x: Wrapping<u8>,
}

impl Rng {
    /// generate a seeded Rng instance. The seed is generated
    /// by calculating the duration since the Unix Epoch  and
    /// taking the `subsec_nanos` value and casting it to a u8
    pub fn default() -> Rng {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Rng {
            seed: Wrapping(seed.subsec_nanos() as u8),
            x: Wrapping(0),
            w: Wrapping(0)
        }
    }
    /// generate a new Rng seeded with the provided
    /// value.
    pub fn new(seed: u8) -> Rng {
        Rng {
            seed: Wrapping(seed),
            x: Wrapping(0),
            w: Wrapping(0)
        }
    }
    /// Calculates the next random value
    /// in the sequence.
    pub fn next(&mut self) -> u8 {
        self.w += self.seed;
        self.x *= self.x;
        self.x += self.w;
        (self.x >> 4 | self.x << 4).0
    }
}