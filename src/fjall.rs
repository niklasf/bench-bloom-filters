use crate::Container;
use std::f32::consts::LN_2;

pub struct FjallBloom {
    /// Bit count
    m: usize,
    /// Number of hash functions
    k: usize,
    /// Raw bytes
    bytes: Vec<u8>,
}

impl FjallBloom {
    pub fn with_n_and_total_bits(n: usize, total_bits: usize) -> Self {
        assert!(n > 0);
        assert!(total_bits > 0);

        let bytes = vec![0; total_bits.div_ceil(8)];
        let m = bytes.len() * 8;

        let bits_per_key = m as f32 / n as f32;
        let k = ((bits_per_key * LN_2) as usize).max(1);

        FjallBloom { bytes, m, k }
    }

    pub fn add_hash(&mut self, mut h1: u64) {
        let mut h2 = secondary_hash(h1);

        for i in 1..=(self.k as u64) {
            let idx = (h1 % self.m as u64);
            self.enable_bit(idx as usize);

            h1 = h1.wrapping_add(h2);
            h2 = h2.wrapping_mul(i);
        }
    }

    fn enable_bit(&mut self, idx: usize) {
        let byte = idx / 8;
        let bit = idx % 8;
        self.bytes[byte] |= 0b1000_0000_u8 >> bit;
    }

    pub fn contains_hash(&self, mut h1: u64) -> bool {
        let mut h2 = secondary_hash(h1);

        for i in 1..=(self.k as u64) {
            let idx = (h1 % self.m as u64);
            if !self.is_bit_enabled(idx as usize) {
                return false;
            }

            h1 = h1.wrapping_add(h2);
            h2 = h2.wrapping_mul(i);
        }

        true
    }

    fn is_bit_enabled(&self, idx: usize) -> bool {
        let byte = idx / 8;
        let bit = idx % 8;
        (self.bytes[byte] & (0b1000_0000_u8 >> bit)) != 0
    }
}

fn secondary_hash(h1: u64) -> u64 {
    h1.wrapping_shr(32).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95)
}

impl Container<u64> for FjallBloom {
    fn check(&self, s: &u64) -> bool {
        self.contains_hash(xxhash_rust::xxh3::xxh3_64(&s.to_be_bytes()))
    }
    fn num_hashes(&self) -> usize {
        self.k
    }
    fn new(num_bits: usize, num_items: usize) -> Self {
        // For fairness, round up to nearest multiple of 64 bits like
        // fastbloom.
        Self::with_n_and_total_bits(num_items, num_bits.div_ceil(64) * 64)
    }
    fn extend<I: Iterator<Item = u64>>(&mut self, items: I) {
        for x in items {
            self.add_hash(xxhash_rust::xxh3::xxh3_64(&x.to_be_bytes()));
        }
    }
    fn name() -> &'static str {
        "fjall"
    }
}

#[cfg(test)]
mod tests {
    use rand::RngCore;

    use super::FjallBloom;

    #[test]
    fn test_contains_guaranteed() {
        let n = 100;
        let mut bloom = FjallBloom::with_bits_per_key(n, 10.0);

        let mut rng = rand::thread_rng();
        let hashes: Vec<u64> = (0..n).map(|_| rng.next_u64()).collect();

        for hash in &hashes {
            bloom.add_hash(*hash);
            assert!(bloom.contains_hash(*hash))
        }

        for hash in &hashes {
            assert!(bloom.contains_hash(*hash))
        }
    }
}
