use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use super::single_thread;

/// List slice will be sorted in another thread if its length
/// is greater than the threshold.
const SINGLETHREAD_THRESHOLD: usize = 100_000;

struct ThreadPool {
  job_sender: Option<Sender<Job>>,
  workers: Vec<Worker>,
}

type Job = Box<dyn FnOnce() + Send>;

struct Worker {
  thread: JoinHandle<()>,
}

impl Worker {
  fn new(job_receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
    let thread = std::thread::spawn(move || {
      loop {
        let f = {
          let rx = job_receiver.lock().unwrap();
          match rx.recv() {
            // Channel has been closed.
            Err(_) => {
              return;
            }
            Ok(f) => f,
          }
        };

        f();
      }
    });

    Self { thread }
  }
}

impl ThreadPool {
  fn new(workers: usize) -> Self {
    let (sender, receiver) = channel();

    let job_receiver = Arc::new(Mutex::new(receiver));

    Self {
      workers: (0..workers)
        .map(|_| Worker::new(Arc::clone(&job_receiver)))
        .collect(),

      job_sender: Some(sender),
    }
  }
  fn from_cores() -> Self {
    Self::new(num_cpus::get())
  }

  fn sender(&self) -> ThreadPoolSender {
    ThreadPoolSender::new(self.job_sender.clone().unwrap())
  }

  fn join(mut self) {
    // Close the channel
    let _ = self.job_sender.take();

    for worker in self.workers.into_iter() {
      worker.thread.join().unwrap();
    }
  }
}

struct ThreadPoolSender {
  job_sender: Sender<Job>,
}

impl ThreadPoolSender {
  fn new(job_sender: Sender<Job>) -> Self {
    Self { job_sender }
  }

  fn run<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    self.job_sender.send(Box::new(f)).unwrap();
  }
}

impl Clone for ThreadPoolSender {
  fn clone(&self) -> Self {
    Self {
      job_sender: self.job_sender.clone(),
    }
  }
}

pub fn quicksort<T: std::cmp::PartialOrd + std::fmt::Debug>(xs: &mut [T]) {
  let pool = ThreadPool::from_cores();
  quicksort_impl(xs, pool.sender());
  pool.join();
}

fn quicksort_impl<T: std::cmp::PartialOrd + std::fmt::Debug>(xs: &mut [T], pool: ThreadPoolSender) {
  if xs.len() <= 1 {
    return;
  }

  if xs.len() < SINGLETHREAD_THRESHOLD {
    single_thread::quicksort(xs);
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
    let subset = &mut xs[0..pivot_index];
    if subset.len() < SINGLETHREAD_THRESHOLD {
      single_thread::quicksort(subset);
    } else {
      let mut subset = SlicePtr::new(subset);

      let pool_clone = pool.clone();
      pool.run(move || quicksort_impl(subset.slice_mut::<T>(), pool_clone.clone()));
    }
  }

  if pivot_index < xs.len() {
    xs.swap(pivot_index, pivot_value_index);

    let subset = &mut xs[pivot_index..];

    if subset.len() < SINGLETHREAD_THRESHOLD {
      single_thread::quicksort(subset);
    } else {
      let mut subset = SlicePtr::new(subset);

      let pool_clone = pool.clone();

      pool.run(move || quicksort_impl(subset.slice_mut::<T>(), pool_clone));
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

impl Clone for SlicePtr {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for SlicePtr {}

unsafe impl Send for SlicePtr {}
unsafe impl Sync for SlicePtr {}

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
        assert!(xs[i] <= xs[i + 1], "{} <= {}", xs[i], xs[i + 1]);
      }
    }
  }

  #[test]
  fn big_list() {
    let len = 10_000_000;

    let mut xs: Vec<i32> = (0..len).map(|_| rand::thread_rng().gen()).collect();

    quicksort(&mut xs);

    for i in 0..xs.len() - 1 {
      assert!(xs[i] <= xs[i + 1], "{} <= {}", xs[i], xs[i + 1]);
    }
  }
}
