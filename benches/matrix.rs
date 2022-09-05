#![allow(unused)]

use rand::{distributions::Standard, prelude::Distribution, Rng};
use threads_and_parallelism::matrix::{self, Matrix};

use criterion::{
  black_box, criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode, Throughput,
};

pub fn criterion_benchmark(c: &mut Criterion) {
  let sizes = [128, 256, 512, 1024, 2048, 4096, 8192];

  let mut group = c.benchmark_group("matrix multiply");

  for size in sizes.iter() {
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Linear);
    group.throughput(Throughput::Elements(*size as u64));

    group.bench_with_input(BenchmarkId::new("singlethread", size), size, |b, &size| {
      b.iter(|| {
        // Creating the matrices inside the bench is kinda bad
        let m1 = gen_matrix::<i64>(size);
        let m2 = gen_matrix::<i64>(size);
        matrix::single_thread::multiply(m1, m2)
      });
    });

    group.bench_with_input(BenchmarkId::new("multithread", size), size, |b, &size| {
      b.iter(|| {
        // Creating the matrices inside the bench is kinda bad
        let m1 = gen_matrix::<i64>(size);
        let m2 = gen_matrix::<i64>(size);
        matrix::multi_thread::multiply(m1, m2)
      });
    });
  }

  group.finish();
}

fn gen_matrix<T>(size: usize) -> Matrix<T>
where
  T: Default + Clone + std::ops::Mul + Send,
  Standard: Distribution<T>,
{
  let mut m1 = vec![vec![T::default(); size]; size];

  for row in m1.iter_mut() {
    for value in row.iter_mut() {
      *value = rand::thread_rng().gen()
    }
  }

  m1.into()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
