use rand::Rng;

pub mod multi_thread;
pub mod single_thread;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>
where
  T: std::ops::Mul + Send,
{
  pub elements: Vec<Vec<T>>,
}

impl<T: std::ops::Mul + Default + Copy + Send> Matrix<T> {
  pub fn new(size: usize) -> Self {
    Self {
      elements: vec![vec![T::default(); size]; size],
    }
  }

  pub fn rows(&self) -> usize {
    self.elements.len()
  }
}

impl<T: std::ops::Mul + Send> From<Vec<Vec<T>>> for Matrix<T> {
  fn from(elements: Vec<Vec<T>>) -> Self {
    Self { elements }
  }
}

impl<T: std::ops::Mul + Send + Clone, const N: usize> From<[[T; N]; N]> for Matrix<T> {
  fn from(elements: [[T; N]; N]) -> Self {
    Self {
      elements: elements.into_iter().map(|row| row.to_vec()).collect(),
    }
  }
}

pub fn gen_matrix(size: usize) -> Matrix<i32> {
  let mut m1 = vec![vec![0; size]; size];

  for row in m1.iter_mut() {
    for value in row.iter_mut() {
      *value = rand::thread_rng().gen_range(-1000..=1000);
    }
  }

  m1.into()
}
