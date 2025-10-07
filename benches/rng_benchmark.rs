use criterion::{Criterion, black_box, criterion_group, criterion_main};
use nova_taus88::Taus88;
use rand::rngs::StdRng;
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use rand_xoshiro::Xoshiro128Plus;

fn benchmark_rngs(c: &mut Criterion) {
    let mut group = c.benchmark_group("RNGs");

    group.bench_function("Taus88", |b| {
        let mut rng = <Taus88 as SeedableRng>::from_seed([1; 12]);
        b.iter(|| black_box(rng.next_u32()))
    });

    group.bench_function("StdRng", |b| {
        let mut rng = <StdRng as rand::SeedableRng>::from_seed([1; 32]);
        b.iter(|| black_box(rng.next_u32()))
    });

    group.bench_function("XorShiftRng", |b| {
        let mut rng = <XorShiftRng as SeedableRng>::from_seed([1; 16]);
        b.iter(|| black_box(rng.next_u32()))
    });

    group.bench_function("Xoshiro128Plus", |b| {
        let mut rng = <Xoshiro128Plus as SeedableRng>::from_seed([1; 16]);
        b.iter(|| black_box(rng.next_u32()))
    });

    group.finish();
}

criterion_group!(benches, benchmark_rngs);
criterion_main!(benches);
