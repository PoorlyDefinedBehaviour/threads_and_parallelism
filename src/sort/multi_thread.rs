use super::single_thread;

const SINGLETHREAD_THRESHOLD: usize = 100_000;

pub fn quicksort<T: std::cmp::PartialOrd + std::fmt::Debug>(xs: &mut [T]) {
  if xs.len() <= 1 {
    return;
  }

  if xs.len() < SINGLETHREAD_THRESHOLD {
    // single_thread::quicksort(xs);
    // return;
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
    let subset = &mut xs[0..pivot_index];
    if subset.len() > SINGLETHREAD_THRESHOLD {
      quicksort(subset);
    } else {
      single_thread::quicksort(subset);
    }
  }

  if pivot_index < xs.len() {
    xs.swap(pivot_index, pivot_value_index);

    let subset = &mut xs[pivot_index..];

    if subset.len() > SINGLETHREAD_THRESHOLD {
      quicksort(subset);
    } else {
      single_thread::quicksort(subset);
    }
  }
}

struct SlicePtr {
  ptr: *mut u8,
  len: usize,
}

impl SlicePtr {
  fn new<T>(slice: &[T]) -> Self {
    Self {
      ptr: slice as *const [T] as *mut u8,
      len: slice.len(),
    }
  }

  fn slice_mut<T>(&mut self) -> &mut [T] {
    unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut T, self.len) }
  }
}

unsafe impl Send for SlicePtr {}
unsafe impl Sync for SlicePtr {}

impl Clone for SlicePtr {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for SlicePtr {}

#[cfg(test)]
mod tests {
  use super::*;

  use proptest::prelude::*;

  #[test]
  fn foo() {
    // let mut xs = vec![1413628989, 0];
    let mut xs = vec![0, 0, 0, -1];
    quicksort(&mut xs);
    dbg!(&xs);
  }

  proptest! {
    #[test]
    fn elements_are_sorted_in_ascending_order(mut xs: Vec<i32>){
      // Sort all lists including the empty one.
      quicksort(&mut xs);

      // Ignore empty lists from now on.
      prop_assume!(!xs.is_empty());

      for i in 0..xs.len() - 1 {
        assert!(xs[i] <= xs[i + 1], "{} <= {}", xs[i], xs[i + 1]);
      }
    }
  }

  #[test]
  fn big_list() {
    let len = rand::thread_rng().gen_range(100_000..=1_000_000);

    let mut xs: Vec<i32> = (0..len).map(|_| rand::thread_rng().gen()).collect();

    quicksort(&mut xs);

    for i in 0..xs.len() - 1 {
      assert!(xs[i] <= xs[i + 1], "{} <= {}", xs[i], xs[i + 1]);
    }
  }
}
