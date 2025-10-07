use core::simd::u32x4;
use rand_core::{RngCore, SeedableRng};

#[repr(transparent)]
pub struct Taus88 {
    pub(crate) z: u32x4,
}

impl Taus88 {
    pub fn new(seed1: u32, seed2: u32, seed3: u32) -> Self {
        assert!(seed1 >= 2, "seed1 must be >= 2");
        assert!(seed2 >= 8, "seed2 must be >= 8");
        assert!(seed3 >= 16, "seed3 must be >= 16");
        Taus88 {
            z: u32x4::from_array([seed1, seed2, seed3, 0]),
        }
    }
}

impl RngCore for Taus88 {
    fn next_u32(&mut self) -> u32 {
        let mask = u32x4::from_array([0xFFFFFFFE, 0xFFFFFFF8, 0xFFFFFFF0, 0]);
        let shift1_l = u32x4::from_array([12, 4, 17, 0]);
        let shift2_l = u32x4::from_array([13, 2, 3, 0]);
        let shift2_r = u32x4::from_array([19, 25, 11, 0]);

        let z = self.z;
        let term1 = (z & mask) << shift1_l;
        let term2_1 = z << shift2_l;
        let term2_2 = (term2_1 ^ z) >> shift2_r;
        let new_z = term1 ^ term2_2;
        self.z = new_z;

        let new_z_lanes = new_z.as_array();
        new_z_lanes[0] ^ new_z_lanes[1] ^ new_z_lanes[2]
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

        Taus88::new(z1.max(2), z2.max(8), z3.max(16))
    }
}
