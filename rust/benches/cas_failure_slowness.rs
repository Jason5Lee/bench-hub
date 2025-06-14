//! Demonstrating that a fail CAS operation can be slower than a success case.

use std::sync::atomic::{AtomicUsize, Ordering};

use criterion::{Criterion, criterion_group, criterion_main};

fn cas_failure_slowness(c: &mut Criterion) {
    c.bench_function("seqcst_success", |b| {
        let value = AtomicUsize::new(0);
        let mut curr_value: usize = 0;
        b.iter(|| {
            let new_value = 1 - curr_value;
            let _ =
                value.compare_exchange(curr_value, new_value, Ordering::SeqCst, Ordering::SeqCst);
            curr_value = new_value;
        })
    });
    c.bench_function("acqrel_success", |b| {
        let value = AtomicUsize::new(0);
        let mut curr_value: usize = 0;
        b.iter(|| {
            let new_value = 1 - curr_value;
            let _ =
                value.compare_exchange(curr_value, new_value, Ordering::AcqRel, Ordering::Acquire);
            curr_value = new_value;
        })
    });
    c.bench_function("seqcst_failure", |b| {
        let value = AtomicUsize::new(0);
        b.iter(|| {
            let _ = value.compare_exchange(1, 0, Ordering::SeqCst, Ordering::SeqCst);
        })
    });
    c.bench_function("acqrel_failure", |b| {
        let value = AtomicUsize::new(0);
        b.iter(|| {
            let _ = value.compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire);
        })
    });
    c.bench_function("acqrel_weak_failure", |b| {
        let value = AtomicUsize::new(0);
        b.iter(|| {
            let _ = value.compare_exchange_weak(1, 0, Ordering::AcqRel, Ordering::Acquire);
        })
    });
}

criterion_group!(benches, cas_failure_slowness);
criterion_main!(benches);
