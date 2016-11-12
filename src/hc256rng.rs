use rand::{ SeedableRng, Rng, Rand };
use hc256::Hc256Rng as Inner;


#[derive(Copy, Clone)]
pub struct Hc256Rng(Inner);

impl Rng for Hc256Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0.gen()
    }
}

impl Rand for Hc256Rng {
    fn rand<R: Rng>(other: &mut R) -> Hc256Rng {
        let mut seed = [0; 16];
        for s in seed.iter_mut() {
            *s = other.gen();
        }
        Self::from_seed(&seed)
    }
}

impl<'a> SeedableRng<&'a [u32]> for Hc256Rng {
    fn reseed(&mut self, seed: &'a [u32]) {
        let Hc256Rng(inner) = Self::from_seed(seed);
        self.0 = inner;
    }

    fn from_seed(seed: &'a [u32]) -> Self {
        let (mut key, mut iv) = ([0; 8], [0; 8]);
        key.clone_from_slice(&seed[..8]);
        iv.clone_from_slice(&seed[8..]);
        Hc256Rng(Inner::init(&key, &iv))
    }
}
