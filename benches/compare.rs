use criterion::AxisScale;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::PlotConfiguration;
use criterion::{criterion_group, criterion_main};

use adqselect;
use floydrivest;
use kth::SliceExtKth;
use order_stat::kth;
use pdqselect::select_by;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn floydrivest_benchmark(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("nth_element");
    group.plot_config(plot_config);
    for size in [1000, 10_000, 100_000, 1_000_000].iter() {
        let k = (size / 2) as usize;
        group.bench_with_input(BenchmarkId::new("floydrivest", size), size, |b, &size| {
            let mut v: Vec<u32> = (0..size).collect();
            v.shuffle(&mut thread_rng());
            b.iter_batched_ref(
                || v.clone(),
                |w: &mut std::vec::Vec<u32>| {
                    floydrivest::nth_element(w, k, &mut Ord::cmp);
                    assert_eq!(w[k], k as u32)
                },
                criterion::BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("order_stat", size), size, |b, &size| {
            let mut v: Vec<u32> = (0..size).collect();
            v.shuffle(&mut thread_rng());
            b.iter_batched_ref(
                || v.clone(),
                |w: &mut std::vec::Vec<u32>| {
                    kth(w, k);
                    assert_eq!(w[k], k as u32)
                },
                criterion::BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("kth", size), size, |b, &size| {
            let mut v: Vec<u32> = (0..size).collect();
            v.shuffle(&mut thread_rng());
            b.iter_batched_ref(
                || v.clone(),
                |w: &mut std::vec::Vec<u32>| {
                    w.partition_by_kth(k);
                    assert_eq!(w[k], k as u32)
                },
                criterion::BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("pdqselect", size), size, |b, &size| {
            let mut v: Vec<u32> = (0..size).collect();
            v.shuffle(&mut thread_rng());
            b.iter_batched_ref(
                || v.clone(),
                |w: &mut std::vec::Vec<u32>| {
                    select_by(w, k, &mut Ord::cmp);
                    assert_eq!(w[k], k as u32)
                },
                criterion::BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("adqselect", size), size, |b, &size| {
            let mut v: Vec<u32> = (0..size).collect();
            v.shuffle(&mut thread_rng());
            b.iter_batched_ref(
                || v.clone(),
                |w: &mut std::vec::Vec<u32>| {
                    adqselect::nth_element(w, k, &mut Ord::cmp);
                    assert_eq!(w[k], k as u32)
                },
                criterion::BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, floydrivest_benchmark);
criterion_main!(benches);
