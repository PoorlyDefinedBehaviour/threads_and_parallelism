use super::Matrix;

pub fn multiply<T: std::ops::Mul<Output = T> + Send + Default + Copy>(
  m1: &Matrix<T>,
  m2: &Matrix<T>,
) -> Matrix<T> {
  let m1_ptr = Ptr(m1 as *const Matrix<T> as *mut u8);
  let m2_ptr = Ptr(m2 as *const Matrix<T> as *mut u8);

  let num_rows = m1.rows();

  let mut out = m1.clone();

  let mut out_ptr = Ptr(&mut out as *mut Matrix<T> as *mut u8);

  let num_cpus = num_cpus::get();

  let subset_length = (num_rows as f64 / num_cpus as f64).floor() as usize + 1;

  let mut threads = Vec::with_capacity(num_cpus - 1);

  let mut i = 0;

  while i < num_rows {
    let subset_starts_at_row = i;

    let subset_ends_at = subset_starts_at_row + subset_length;

    threads.push(std::thread::spawn(move || {
      for i in subset_starts_at_row..std::cmp::min(subset_ends_at, num_rows) {
        let m1: &Matrix<T> = m1_ptr.get_matrix();
        let m2: &Matrix<T> = m2_ptr.get_matrix();
        let out: &mut Matrix<T> = out_ptr.get_matrix_mut();

        for j in 0..num_rows {
          out.elements[j][i] = m1.elements[j][i] * m2.elements[i][j];
        }
      }
    }));

    i = subset_ends_at;

    if threads.len() == num_cpus - 1 {
      for i in subset_starts_at_row..std::cmp::min(subset_ends_at, num_rows) {
        let m1: &Matrix<T> = m1_ptr.get_matrix();
        let m2: &Matrix<T> = m2_ptr.get_matrix();
        let out: &mut Matrix<T> = out_ptr.get_matrix_mut();

        for j in 0..num_rows {
          out.elements[j][i] = m1.elements[j][i] * m2.elements[i][j];
        }
      }

      break;
    }
  }

  for thread in threads.into_iter() {
    thread.join().unwrap();
  }

  out
}

struct Ptr(*mut u8);

impl Ptr {
  fn get_matrix<T: std::ops::Mul + Send>(&self) -> &Matrix<T> {
    unsafe { &(*(self.0 as *mut Matrix<T>)) }
  }

  fn get_matrix_mut<T: std::ops::Mul + Send>(&mut self) -> &mut Matrix<T> {
    unsafe { &mut (*(self.0 as *mut Matrix<T>)) }
  }
}

unsafe impl Send for Ptr {}
unsafe impl Sync for Ptr {}

impl Clone for Ptr {
  fn clone(&self) -> Self {
    Self(self.0)
  }
}

impl Copy for Ptr {}

#[cfg(test)]
mod tests {
  use std::time::Instant;

  use crate::matrix::gen_matrix;

  use super::*;

  #[test]
  fn test_multiply() {
    assert_eq!(
      multiply(
        &vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]].into(),
        &vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]].into()
      ),
      vec![vec![9, 12, 9], vec![32, 25, 12], vec![49, 32, 9]].into()
    );
  }

  #[ignore]
  #[test]
  fn debug() {
    let m1: Matrix<i32> = gen_matrix(8192);
    let m2: Matrix<i32> = gen_matrix(8192);

    let start = Instant::now();
    let _ = multiply(&m1, &m2);
    println!("elapsed={:?}", start.elapsed());
  }
}
