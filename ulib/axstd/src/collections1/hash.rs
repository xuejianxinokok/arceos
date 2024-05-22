
pub struct DefaultHasher {
    seed: u32,
}

impl DefaultHasher {
    pub fn new() -> Self {
        DefaultHasher {
            seed: time::current_ticks() as u32,
        }
    }

    pub fn hash(&mut self, keys: &[u8]) -> u64 {
        let mut hash: u64 = 0;
        for _ in 0..4 {
            self.seed = (self.seed as u64 * 48271 % RAND_MAX) as u32;
            hash = (hash << 32) | (self.seed as u64);
        }
        hash ^ simple_hash(keys)
    }
}

fn simple_hash(keys: &[u8]) -> u64 {
    let mut hash: u64 = 0;
    for c in keys {
        hash = (hash << 5).wrapping_sub(hash).wrapping_add((*c).into());
    }
    hash
}




use spinlock::SpinNoIrq;
use crate::time;

static PARK_MILLER_LEHMER_SEED: SpinNoIrq<u32> = SpinNoIrq::new(0);
const RAND_MAX: u64 = 2_147_483_647;

pub fn random() -> u128 {
    let mut seed = PARK_MILLER_LEHMER_SEED.lock();
    if *seed == 0 {
        *seed = time::current_ticks() as u32;
    }

    let mut ret: u128 = 0;
    for _ in 0..4 {
        *seed = ((u64::from(*seed) * 48271) % RAND_MAX) as u32;
        ret = (ret << 32) | (*seed as u128);
    }
    ret
}
