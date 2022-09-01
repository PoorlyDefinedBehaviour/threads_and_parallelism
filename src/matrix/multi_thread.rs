use super::Matrix;

pub fn multiply<T: std::ops::Mul<Output = T> + Send + Default + Copy>(
  m1: impl Into<Matrix<T>>,
  m2: impl Into<Matrix<T>>,
) -> Matrix<T> {
  /*
  [
    [1, 2, 3], | thread 1
    [4, 5, 6], | thread 1
    [7, 8, 9], | thread 2
    [0, 4, 10] | thread 2
  ]

  [
    [9, 8, 7], | thread 1
    [6, 5, 4], | thread 1
    [3, 2, 1], | thread 2
    [4, 6, 12] | thread 2
  ]
  */
  let mut m1 = m1.into();
  let mut m2 = m2.into();

  let m1_ptr = Ptr(&mut m1 as *mut Matrix<T> as *mut u8);
  let m2_ptr = Ptr(&mut m2 as *mut Matrix<T> as *mut u8);

  let num_rows = m1.rows();

  let mut out = Matrix::new(num_rows);

  let mut out_ptr = Ptr(&mut out as *mut Matrix<T> as *mut u8);

  let num_cpus = num_cpus::get();

  let subset_length = (num_rows as f64 / num_cpus as f64).floor() as usize + 1;

  let mut threads = Vec::with_capacity(num_cpus);

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
  use super::*;

  #[test]
  fn test_multiply() {
    assert_eq!(
      multiply(
        [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        [[9, 8, 7], [6, 5, 4], [3, 2, 1]]
      ),
      [[9, 12, 9], [32, 25, 12], [49, 32, 9]].into()
    );
  }
}
