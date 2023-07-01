const KN: usize = 624;
const KM: usize = 397;
const KMAG: &'static [u32] = &[0, 0x9908b0df];
const KUPPERMASK: u32 = 0x80000000;
const KLOWERMASK: u32 = 0x7fffffff;

pub struct MT19937 {
    current_index_: usize,
    data_: Vec<u32>,
}

impl MT19937 {
    pub fn new(seed: u32) -> MT19937 {
        let mut engine = MT19937 {
            current_index_: 1,
            data_: vec![0; KN],
        };
        engine.data_[0] = seed & 0xFFFFFFFF;
        while engine.current_index_ < KN {
            engine.data_[engine.current_index_] = engine.data_[engine.current_index_ - 1];
            engine.data_[engine.current_index_] ^= engine.data_[engine.current_index_ - 1] >> 30;
            engine.data_[engine.current_index_] =
                engine.data_[engine.current_index_].wrapping_mul(1812433253);
            engine.data_[engine.current_index_] =
                engine.data_[engine.current_index_].wrapping_add(engine.current_index_ as u32);
            engine.data_[engine.current_index_] &= 0xFFFFFFFF;
            engine.current_index_ += 1;
        }
        return engine;
    }

    pub fn gen_rand_uint32(&mut self) -> u32 {
        let mut y: u32;
        if self.current_index_ >= KN {
            let mut i: usize = 0;
            // TODO error if new wasn't invoked
            while i < KN - KM {
                y = self.data_[i] & KUPPERMASK;
                y |= self.data_[i + 1] & KLOWERMASK;
                self.data_[i] = self.data_[i + KM];
                self.data_[i] ^= y >> 1;
                self.data_[i] ^= KMAG[(y & 1) as usize];
                i += 1;
            }
            while i < KN - 1 {
                y = self.data_[i] & KUPPERMASK;
                y |= self.data_[i + 1] & KLOWERMASK;
                self.data_[i] = self.data_[i - 227];
                self.data_[i] ^= y >> 1;
                self.data_[i] ^= KMAG[(y & 1) as usize];
                i += 1;
            }
            y = self.data_[KN - 1] & KUPPERMASK;
            y |= self.data_[0] & KLOWERMASK;
            self.data_[KN - 1] = self.data_[KM - 1];
            self.data_[i] ^= y >> 1;
            self.data_[i] ^= KMAG[(y & 1) as usize];
	    self.current_index_ = 0;
        }
        y = self.data_[self.current_index_];
	self.current_index_ += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;
        return y;
    }
}

pub fn generate_mt19937(seed: u32) -> Box<MT19937> {
    return Box::new(MT19937::new(seed));
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn fails_on_bad_seed() {
//         let bad_seed = 10;
//         let invalid_rng = crate::generate_mt19937(bad_seed);
//         assert!(invalid_rng.is_none());
//     }
// }
