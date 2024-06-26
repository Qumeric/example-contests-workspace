use crate::collections::slice_ext::indices::Indices;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::ops::Rem;
use std::time::SystemTime;

const NN: usize = 312;
const MM: usize = 156;
const MATRIX_A: u64 = 0xB5026F5AA96619E9;
const UM: u64 = 0xFFFFFFFF80000000;
const LM: u64 = 0x7FFFFFFF;
const F: u64 = 6364136223846793005;
const MAG01: [u64; 2] = [0, MATRIX_A];

pub struct Random {
    mt: [u64; NN],
    index: usize,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        let mut res = Self {
            mt: [0u64; NN],
            index: NN,
        };
        res.mt[0] = seed;
        for i in 1..NN {
            res.mt[i] = F
                .wrapping_mul(res.mt[i - 1] ^ (res.mt[i - 1] >> 62))
                .wrapping_add(i as u64);
        }
        res
    }

    pub fn gen(&mut self) -> u64 {
        if self.index == NN {
            for i in 0..(NN - MM) {
                let x = (self.mt[i] & UM) | (self.mt[i + 1] & LM);
                self.mt[i] = self.mt[i + MM] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
            }
            for i in (NN - MM)..(NN - 1) {
                let x = (self.mt[i] & UM) | (self.mt[i + 1] & LM);
                self.mt[i] = self.mt[i + MM - NN] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
            }
            let x = (self.mt[NN - 1] & UM) | (self.mt[0] & LM);
            self.mt[NN - 1] = self.mt[MM - 1] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
            self.index = 0;
        }
        let mut x = self.mt[self.index];
        self.index += 1;
        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71D67FFFEDA60000;
        x ^= (x << 37) & 0xFFF7EEE000000000;
        x ^= x >> 43;
        x
    }

    pub fn next<T: Rem<Output = T> + Primitive<u64>>(&mut self, n: T) -> T
    where
        u64: Primitive<T>,
    {
        (self.gen() % n.to()).to()
    }

    pub fn next_bounds<T: Rem<Output = T> + AddSub + ZeroOne + Primitive<u64>>(
        &mut self,
        f: T,
        t: T,
    ) -> T
    where
        u64: Primitive<T>,
    {
        f + self.next(t - f + T::one())
    }
}

static mut RAND: Option<Random> = None;

pub fn random() -> &'static mut Random {
    unsafe {
        if RAND.is_none() {
            RAND = Some(Random::new(
                (SystemTime::UNIX_EPOCH.elapsed().unwrap().as_nanos() & 0xFFFFFFFFFFFFFFFF) as u64,
            ));
        }
        RAND.as_mut().unwrap()
    }
}

pub trait Shuffle {
    fn shuffle(&mut self);
}

impl<T> Shuffle for [T] {
    fn shuffle(&mut self) {
        for i in self.indices() {
            let at = random().next(i + 1);
            self.swap(i, at);
        }
    }
}
