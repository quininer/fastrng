#![feature(test)]

extern crate test;
extern crate rand;
extern crate fastrng;

use test::Bencher;
use rand::{ Rng, ChaChaRng, IsaacRng, thread_rng };
use fastrng::Hc256Rng;


macro_rules! bench_rng {
    ( $name:ident $ty:ident $len:expr ) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng = thread_rng().gen::<$ty>();
            b.bytes = $len;
            b.iter(|| {
                let mut data = [0; $len];
                rng.fill_bytes(&mut data);
            })
        }
    }
}

bench_rng!(chacharng_bench_1k   ChaChaRng   1024);
bench_rng!(isaacrng_bench_1k    IsaacRng    1024);
bench_rng!(hc256rng_bench_1k    Hc256Rng    1024);
