use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use rand::prelude::*;
use shishua::ShiShuARng;

pub fn benchmark_shisuha(c: &mut Criterion) {
    let length = 1024 * 1024;
    let mut rng = ShiShuARng::from_os_rng();
    let mut bytes = vec![0; length];
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function("filling 1MB", |b| {
        b.iter(|| rng.fill(bytes.as_mut_slice()))
    });
    group.finish();
}

criterion_group!(benches, benchmark_shisuha);
criterion_main!(benches);
