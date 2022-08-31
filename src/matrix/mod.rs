pub mod multi_thread;
pub mod single_thread;

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>
where
  T: std::ops::Mul,
{
  elements: Vec<Vec<T>>,
}

impl<T: std::ops::Mul + Default + Copy> Matrix<T> {
  pub fn new(size: usize) -> Self {
    Self {
      elements: (0..size)
        .map(|_| (0..size).map(|_| T::default()).collect())
        .collect(),
    }
  }

  pub fn rows(&self) -> usize {
    self.elements.len()
  }
}

impl<T: std::ops::Mul> From<Vec<Vec<T>>> for Matrix<T> {
  fn from(elements: Vec<Vec<T>>) -> Self {
    Self { elements }
  }
}

impl<T: std::ops::Mul + Clone, const N: usize> From<[[T; N]; N]> for Matrix<T> {
  fn from(elements: [[T; N]; N]) -> Self {
    Self {
      elements: elements.into_iter().map(|row| row.to_vec()).collect(),
    }
  }
}
