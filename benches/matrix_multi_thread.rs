#![allow(unused)]

use rand::{distributions::Standard, prelude::Distribution, Rng};
use threads_and_parallelism::matrix::{multi_thread::multiply, Matrix};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

pub fn criterion_benchmark(c: &mut Criterion) {
  let sizes = [128, 256, 512, 1024, 2048];

  let mut group = c.benchmark_group("multi_thread_multiply_i64");
  for size in sizes.iter() {
    group.throughput(Throughput::Elements(*size as u64));
    group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
      b.iter(|| {
        // Creating the matrices inside the bench is kinda bad
        let m1 = gen_matrix::<i64>(size);
        let m2 = gen_matrix::<i64>(size);
        multiply(m1, m2)
      });
    });
  }
  group.finish();

  let mut group = c.benchmark_group("multi_thread_multiply_f64");
  for size in sizes.iter() {
    group.throughput(Throughput::Elements(*size as u64));
    group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
      b.iter(|| {
        // Creating the matrices inside the bench is kinda bad
        let m1 = gen_matrix::<f64>(size);
        let m2 = gen_matrix::<f64>(size);
        multiply(m1, m2)
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
