#![feature(macro_rules)]

pub struct Slide<T: Iterator<A>, A> {
  iter: T,
  n: uint,
  window: Vec<A>
}

macro_rules! return_if(
  ($cond:expr, $value:expr) => (
    if $cond {
      return $value;
    }
  );
)

impl<A: Clone, T: Iterator<A>> Slide<T, A> {
  fn push_window(&mut self) -> bool {
    let iter_next = self.iter.next();

    if iter_next.is_some() {
      self.window.push(iter_next.unwrap());
      true
    } else {
      false
    }
  }

  fn new(iter: T, n: uint) -> Slide<T, A> {
    Slide{
      iter: iter,
      n: n,
      window: Vec::with_capacity(n)
    }
  }
}

impl<A: Clone, T: Iterator<A>> Iterator<Vec<A>> for Slide<T, A> {
  fn next(&mut self) -> Option<Vec<A>> {
    return_if!(self.n == 0, None);
    return_if!(!self.push_window(), None);

    loop {
      let window_status = self.window.len().cmp(&self.n);

      match window_status {
        Greater => { self.window.remove(0); }
        Equal => { return Some(self.window.clone()); }
        Less => { return_if!(!self.push_window(), None); }
      }
    }
  }
}

pub trait SlideIterator<T: Iterator<A>, A> {
  fn slide(self, n: uint) -> Slide<T, A>;
}

impl<A: Clone, T: Iterator<A>> SlideIterator<T, A> for T {
  fn slide(self, n: uint) -> Slide<T, A> {
    Slide::new(self, n)
  }
}

#[test]
fn test_slide() {
  let mut slide_iter = vec![1i, 2, 3, 4, 5].into_iter().slide(3);
  assert_eq!(slide_iter.next().unwrap(), vec![1, 2, 3])
  assert_eq!(slide_iter.next().unwrap(), vec![2, 3, 4])
  assert_eq!(slide_iter.next().unwrap(), vec![3, 4, 5])
  assert!(slide_iter.next().is_none())
}

#[test]
fn test_slide_equal_window() {
  let mut slide_iter = vec![1i, 2, 3, 4, 5].into_iter().slide(5);
  assert_eq!(slide_iter.next().unwrap(), vec![1, 2, 3, 4, 5])
  assert!(slide_iter.next().is_none())
}

#[test]
fn test_slide_zero_window() {
  let mut slide_iter = vec![1i, 2, 3, 4, 5].into_iter().slide(0);
  assert!(slide_iter.next().is_none())
}

#[test]
fn test_slide_overlong_window() {
  let mut slide_iter = vec![1i, 2, 3, 4, 5].into_iter().slide(7);
  assert!(slide_iter.next().is_none())
}
