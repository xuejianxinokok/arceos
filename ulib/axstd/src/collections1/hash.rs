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




pub struct DefaultHasher ;

impl DefaultHasher {
    pub fn new() -> Self {
        DefaultHasher {
        }
    }

    pub fn hash(& self, keys: &[u8]) -> u128 {
        random() ^ simple_hash(keys)
    }
}

fn simple_hash(keys: &[u8]) -> u128 {
    let mut hash: u128 = 0;
    for c in keys {
        hash = (hash << 5).wrapping_sub(hash).wrapping_add((*c).into());
    }
    hash
}




