pub fn quicksort<T: std::cmp::PartialOrd + std::fmt::Debug>(xs: &mut [T]) {
  if xs.len() <= 1 {
    return;
  }

  let mut pivot_index = 0;
  let pivot_value_index = xs.len() - 1;

  for i in 0..xs.len() {
    if xs[i] <= xs[pivot_value_index] {
      xs.swap(pivot_index, i);

      pivot_index += 1;
    }
  }

  if pivot_index >= xs.len() {
    pivot_index -= 1;
  }

  if pivot_index > 0 {
    quicksort(&mut xs[0..pivot_index]);
  }

  if pivot_index < xs.len() {
    xs.swap(pivot_index, pivot_value_index);
    quicksort(&mut xs[pivot_index..]);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use proptest::prelude::*;

  proptest! {
    #[test]
    fn elements_are_sorted_in_ascending_order(mut xs: Vec<i32>){
      // Sort all lists including the empty one.
      quicksort(&mut xs);

      // Ignore empty lists from now on.
      prop_assume!(!xs.is_empty());

      for i in 0..xs.len() - 1 {
        assert!(xs[i] <= xs[i + 1], "xs = {:?} {} <= {}", &xs, xs[i], xs[i + 1]);
      }
    }
  }
}
