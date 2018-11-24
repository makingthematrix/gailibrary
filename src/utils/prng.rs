use rand::{Rng, SeedableRng, XorShiftRng};
use std;

#[derive(Debug, Clone)]
pub struct PseudoNormalRng {
    a: f64,
    rng: XorShiftRng,
}

#[inline]
fn polynomial(x: f64, a: f64) -> f64 {
    a * x.powi(3) + (1.0 - a) * x
}

#[inline]
pub fn pseudo_normalize(x: f64, a: f64) -> f64 {
    debug_assert!(x >= 0.0 && x <= 1.0);
    debug_assert!(a >= 0.0);

    let mut my_x = x * 2.0 - 1.0;
    let mut my_a = a;

    while my_a > 1.0 {
        my_x = polynomial(my_x, 1.0);
        my_a -= 1.0;
    }

    my_x = polynomial(my_x, my_a);
    (my_x + 1.0) / 2.0
}

fn new_seed() -> [u32; 4] {
    let s1: u32 = rand::random();
    let s2: u32 = rand::random();
    let s3: u32 = rand::random();
    let s4: u32 = rand::random();
    [s1, s2, s3, s4]
}

impl PseudoNormalRng {
    #[inline]
    pub fn with_seed(a: f64, seed: [u32; 4]) -> Self {
        PseudoNormalRng {
            a,
            rng: rand::XorShiftRng::from_seed(seed),
        }
    }

    #[inline]
    pub fn new(a: f64) -> Self {
        Self::with_seed(a, new_seed())
    }

    pub fn gen_range(&mut self, low: usize, high: usize) -> usize {
        debug_assert!(low <= high);
        (pseudo_normalize(self.rng.next_f64(), self.a) * (high - low) as f64) as usize + low
    }
}

impl Rng for PseudoNormalRng {
    fn next_u32(&mut self) -> u32 {
        let max = f64::from(std::u32::MAX);
        (pseudo_normalize(self.rng.next_f64(), self.a) * max) as u32
    }

    fn next_u64(&mut self) -> u64 {
        (pseudo_normalize(self.rng.next_f64(), self.a) * std::u64::MAX as f64) as u64
    }

    fn next_f32(&mut self) -> f32 {
        pseudo_normalize(self.rng.next_f64(), self.a) as f32
    }

    fn next_f64(&mut self) -> f64 {
        pseudo_normalize(self.rng.next_f64(), self.a)
    }
}

impl SeedableRng<[u32; 4]> for PseudoNormalRng {
    fn reseed(&mut self, seed: [u32; 4]) {
        self.rng.reseed(seed)
    }

    fn from_seed(seed: [u32; 4]) -> Self {
        PseudoNormalRng {
            a: 1.0,
            rng: rand::XorShiftRng::from_seed(seed),
        }
    }
}
