use rand_core::{RngCore, SeedableRng};

/// A Tausworthe-based random number generator (Taus88).
///
/// The state must be seeded with values that meet the algorithm's minimums
/// to ensure a long period and prevent state collapse.
pub struct Taus88 {
    pub(crate) z1: u32,
    pub(crate) z2: u32,
    pub(crate) z3: u32,
}

impl Taus88 {
    /// Creates a new `Taus88` instance with the given seeds.
    ///
    /// The seeds must meet the following conditions to prevent state collapse:
    /// - `seed1` must be >= 2
    /// - `seed2` must be >= 8
    /// - `seed3` must be >= 16
    pub fn new(seed1: u32, seed2: u32, seed3: u32) -> Self {
        assert!(seed1 >= 2, "seed1 must be >= 2");
        assert!(seed2 >= 8, "seed2 must be >= 8");
        assert!(seed3 >= 16, "seed3 must be >= 16");
        Taus88 {
            z1: seed1,
            z2: seed2,
            z3: seed3,
        }
    }
}

impl RngCore for Taus88 {
    fn next_u32(&mut self) -> u32 {
        self.z1 = ((self.z1 & 0xFFFFFFFE) << 12) ^ (((self.z1 << 13) ^ self.z1) >> 19);
        self.z2 = ((self.z2 & 0xFFFFFFF8) << 4) ^ (((self.z2 << 2) ^ self.z2) >> 25);
        self.z3 = ((self.z3 & 0xFFFFFFF0) << 17) ^ (((self.z3 << 3) ^ self.z3) >> 11);
        self.z1 ^ self.z2 ^ self.z3
    }

    fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 32 | self.next_u32() as u64
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(4) {
            let rand = self.next_u32();
            let bytes = rand.to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len()]);
        }
    }
}

impl SeedableRng for Taus88 {
    type Seed = [u8; 12];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut s1_bytes = [0u8; 4];
        s1_bytes.copy_from_slice(&seed[0..4]);
        let mut s2_bytes = [0u8; 4];
        s2_bytes.copy_from_slice(&seed[4..8]);
        let mut s3_bytes = [0u8; 4];
        s3_bytes.copy_from_slice(&seed[8..12]);

        let z1 = u32::from_le_bytes(s1_bytes);
        let z2 = u32::from_le_bytes(s2_bytes);
        let z3 = u32::from_le_bytes(s3_bytes);

        // Ensure the seeds meet the minimum requirements for the generator by
        // promoting them to the minimum value if they are too low.
        Taus88::new(z1.max(2), z2.max(8), z3.max(16))
    }
}
