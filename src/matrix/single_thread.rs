use super::Matrix;

pub fn multiply<T: std::ops::Mul<Output = T> + Send + Default + Copy>(
  m1: &Matrix<T>,
  m2: &Matrix<T>,
) -> Matrix<T> {
  let mut out = Matrix::new(m1.rows());

  for i in 0..m1.rows() {
    for j in 0..m1.rows() {
      out.elements[j][i] = m1.elements[j][i] * m2.elements[i][j];
    }
  }

  out
}

pub fn positional_multiply<T: std::ops::Mul<Output = T> + Send + Default + Copy>(
  m1: &Matrix<T>,
  m2: &Matrix<T>,
) -> Matrix<T> {
  let mut out = Matrix::new(m1.rows());

  for i in 0..m1.rows() {
    for j in 0..m1.rows() {
      out.elements[i][j] = m1.elements[i][j] * m2.elements[i][j];
    }
  }

  out
}

#[cfg(test)]
mod tests {
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

  #[test]
  fn test_positional_multiply() {
    assert_eq!(
      positional_multiply(
        &vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]].into(),
        &vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]].into()
      ),
      vec![vec![9, 16, 21], vec![24, 25, 24], vec![21, 16, 9]].into()
    );
  }
}
