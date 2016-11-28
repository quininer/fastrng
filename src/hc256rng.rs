use rand::{ SeedableRng, Rng, Rand };
use hc256::Hc256Rng as InnerRng;


#[derive(Clone)]
pub struct Hc256Rng(InnerRng);

impl Rng for Hc256Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0.gen()
    }
}

impl Rand for Hc256Rng {
    fn rand<R: Rng>(other: &mut R) -> Hc256Rng {
        let mut seed = [0; 16];
        for s in &mut seed {
            *s = other.gen();
        }
        Self::from_seed(&seed)
    }
}

impl<'a> SeedableRng<&'a [u32]> for Hc256Rng {
    #[inline]
    fn reseed(&mut self, seed: &'a [u32]) {
        *self = Self::from_seed(seed);
    }

    fn from_seed(seed: &'a [u32]) -> Self {
        let mut w = [0; 2560];
        w[..16].clone_from_slice(seed);
        Hc256Rng(InnerRng::with_w(&mut w))
    }
}
