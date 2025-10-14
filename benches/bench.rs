use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use rand::prelude::*;
use shishua::ShiShuARng;


#[cfg(feature = "__intern_c_bindings")]
extern "C" {
    fn shishua_bindings_init(seed: *const u64) -> *mut ();
    fn shishua_bindings_destroy(state: *mut ());
    fn shishua_bindings_generate(state: *mut (), buffer: *mut u8, size: usize);
}


pub fn benchmark_shisuha(c: &mut Criterion) {
    const KB: usize = 1024;
    const MB: usize = 1024 * 1024;

    let seed = [0x1, 0x2, 0x3, 0x4];

    let mut rng = ShiShuARng::new(seed);
    #[cfg(feature = "__intern_c_bindings")]
    let native_rng = unsafe { shishua_bindings_init(seed.as_ptr()) };

    let mut group = c.benchmark_group("throughput");

    for size in [512, KB, MB] {
        assert_eq!(size % 512, 0);

        let mut buffer = vec![0; size];

        group.throughput(Throughput::Bytes(size as u64));

        #[cfg(all(not(feature = "nightly"), not(feature = "wide")))]
        const SHISHUARS_NAME: &str = "shishua_rs_soft";
        #[cfg(all(not(feature = "nightly"), feature = "wide"))]
        const SHISHUARS_NAME: &str = "shishua_rs_wide";
        #[cfg(feature = "nightly")]
        const SHISHUARS_NAME: &str = "shishua_rs_nightly";

        group.bench_function(BenchmarkId::new(SHISHUARS_NAME, size), |b| {
            b.iter(|| rng.fill(buffer.as_mut_slice()))
        });

        #[cfg(feature = "__intern_c_bindings")]
        group.bench_function(BenchmarkId::new("shishua_c", size), |b| {
            b.iter(|| unsafe {
                shishua_bindings_generate(native_rng, buffer.as_mut_ptr(), size)
            });
        });
    }

    #[cfg(feature = "__intern_c_bindings")]
    unsafe {shishua_bindings_destroy(native_rng)};
    group.finish();
}

criterion_group!(benches, benchmark_shisuha);
criterion_main!(benches);
