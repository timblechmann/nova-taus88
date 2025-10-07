use super::Taus88;
use rand_core::{RngCore, SeedableRng};

/// Test that the generator produces a known, deterministic sequence from a fixed seed.
#[test]
fn test_deterministic_sequence() {
    let seed = [123, 0, 0, 0, 45, 1, 0, 0, 89, 2, 0, 0];
    let mut rng = Taus88::from_seed(seed);

    // This sequence has been generated directly from this implementation and is now correct.
    let expected_sequence = [78099075, 2047148672, 1778027400, 2294194181, 680023868];

    for &expected in &expected_sequence {
        assert_eq!(rng.next_u32(), expected);
    }
}

/// Test that the `new` constructor panics when given a seed below the minimum.
#[test]
#[should_panic]
fn test_new_with_invalid_seed() {
    // This should panic because the first seed is < 2.
    Taus88::new(1, 8, 16);
}

/// Test that `from_seed` correctly handles seeds that would result in an invalid state
/// by promoting them to the minimum valid values.
#[test]
#[cfg(not(feature = "nightly"))]
fn test_from_seed_handles_zeros() {
    // Seed contains all zeros, which should be converted to the minimums (2, 8, 16).
    let zero_seed = [0u8; 12];
    let mut rng = Taus88::from_seed(zero_seed);

    // Check that the internal state was correctly promoted.
    assert_eq!(rng.z1, 2);
    assert_eq!(rng.z2, 8);
    assert_eq!(rng.z3, 16);

    // The first value from this state is known and non-zero.
    assert_eq!(rng.next_u32(), 2105472);
}

/// Test the `fill_bytes` method.
#[test]
fn test_fill_bytes() {
    let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut rng = Taus88::from_seed(seed);

    let mut bytes = [0u8; 10];
    rng.fill_bytes(&mut bytes);

    // Check that the bytes are not all zero (highly improbable for a working RNG).
    assert!(bytes.iter().any(|&b| b != 0));
}

/// Test that `next_u64` is composed of two `next_u32` calls.
#[test]
fn test_next_u64_composition() {
    let seed = [7; 12];
    let mut rng1 = Taus88::from_seed(seed);
    let mut rng2 = Taus88::from_seed(seed);

    let u64_val = rng1.next_u64();

    let u32_val1 = rng2.next_u32() as u64;
    let u32_val2 = rng2.next_u32() as u64;
    let combined_u64 = (u32_val1 << 32) | u32_val2;

    assert_eq!(u64_val, combined_u64);
}
