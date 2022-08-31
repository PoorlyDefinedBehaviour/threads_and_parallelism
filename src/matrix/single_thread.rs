use super::Matrix;

pub fn multiply<T: std::ops::Mul<Output = T> + Default + Copy>(
  m1: impl Into<Matrix<T>>,
  m2: impl Into<Matrix<T>>,
) -> Matrix<T> {
  let m1 = m1.into();
  let m2 = m2.into();

  let mut out = Matrix::new(m1.rows());

  for i in 0..m1.rows() {
    for j in 0..m1.rows() {
      out.elements[j][i] = m1.elements[j][i] * m2.elements[i][j];
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
        [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        [[9, 8, 7], [6, 5, 4], [3, 2, 1]]
      ),
      [[9, 12, 9], [32, 25, 12], [49, 32, 9]].into()
    );
  }
}
