use rand::prelude::*;
use rand::rngs::ThreadRng as RNG;

pub struct RandomNumbers {
    rng: RNG,
    min: u64,
    max: u64,
}

impl RandomNumbers {
    pub fn new(min: u64, max: u64) -> Self {
        Self {
            rng: thread_rng(),
            min,
            max,
        }
    }
}

impl Iterator for RandomNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.gen_range(self.min..self.max))
    }
}
