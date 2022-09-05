#![allow(unused)]

use rand::{distributions::Standard, prelude::Distribution, Rng};
use threads_and_parallelism::sort;

use criterion::{
  black_box, criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode, Throughput,
};

pub fn criterion_benchmark(c: &mut Criterion) {
  let lengths = [
    100_000, 500_000, 1_000_000, 5_000_000, 10_000_000, 50_000_000,
  ];

  let mut group = c.benchmark_group("sort");
  for length in lengths.iter() {
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Linear);
    group.throughput(Throughput::Elements(*length as u64));

    let xs: Vec<i64> = gen_vec(*length);

    group.bench_with_input(
      BenchmarkId::new("multithread", length),
      length,
      |b, &length| {
        b.iter(|| {
          sort::multi_thread::quicksort(&mut xs.clone());
        });
      },
    );

    group.bench_with_input(
      BenchmarkId::new("singlethread", length),
      length,
      |b, &length| {
        b.iter(|| {
          sort::single_thread::quicksort(&mut xs.clone());
        });
      },
    );
  }
  group.finish();
}

fn gen_vec<T>(length: usize) -> Vec<T>
where
  Standard: Distribution<T>,
{
  (0..=length).map(|_| rand::thread_rng().gen()).collect()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
