use criterion::{Criterion, criterion_group, criterion_main};
use rand::rngs::StdRng;
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use rand_xoshiro::Xoshiro128Plus;
use std::hint::black_box;

fn benchmark_rngs(c: &mut Criterion) {
    let mut group = c.benchmark_group("RNGs");

    group.bench_function("Taus88: Scalar", |b| {
        let mut rng = <nova_taus88::scalar::Taus88 as SeedableRng>::from_seed([1; 12]);
        let mut buf = [0u8; 128];
        b.iter(|| {
            rng.fill_bytes(&mut buf);
            black_box(&buf);
        })
    });

    #[cfg(feature = "nightly")]
    group.bench_function("Taus88: Simd", |b| {
        let mut rng = <nova_taus88::simd::Taus88 as SeedableRng>::from_seed([1; 12]);
        let mut buf = [0u8; 128];
        b.iter(|| {
            rng.fill_bytes(&mut buf);
            black_box(&buf);
        })
    });

    group.bench_function("StdRng", |b| {
        let mut rng = <StdRng as SeedableRng>::from_seed([1; 32]);
        let mut buf = [0u8; 128];
        b.iter(|| {
            rng.fill_bytes(&mut buf);
            black_box(&buf);
        })
    });

    group.bench_function("XorShiftRng", |b| {
        let mut rng = <XorShiftRng as SeedableRng>::from_seed([1; 16]);
        let mut buf = [0u8; 128];
        b.iter(|| {
            rng.fill_bytes(&mut buf);
            black_box(&buf);
        })
    });

    group.bench_function("Xoshiro128Plus", |b| {
        let mut rng = <Xoshiro128Plus as SeedableRng>::from_seed([1; 16]);
        let mut buf = [0u8; 128];
        b.iter(|| {
            rng.fill_bytes(&mut buf);
            black_box(&buf);
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_rngs);
criterion_main!(benches);
