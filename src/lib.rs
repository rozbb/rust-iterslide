use std::cmp::Ordering::{Greater, Equal, Less};

pub struct Slide<T: Iterator<Item=A>, A> {
    iter: T,
    n: usize,
    window: Vec<A>
}

macro_rules! return_if(
    ($cond:expr, $value:expr) => (
        if $cond {
            return $value;
        }
    );
);

impl<A: Clone, T: Iterator<Item=A>> Slide<T, A> {
    fn push_window(&mut self) -> bool {
        let iter_next = self.iter.next();
        let is_some = iter_next.is_some();

        if is_some {
            self.window.push(iter_next.unwrap());
        }

        is_some
    }

    fn new(iter: T, n: usize) -> Slide<T, A> {
        Slide {
            iter: iter,
            n: n,
            window: Vec::with_capacity(n + 1)
        }
    }
}

impl<A: Clone, T: Iterator<Item=A>> Iterator for Slide<T, A> {
    type Item = Vec<A>;

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

pub trait SlideIterator<T: Iterator<Item=A>, A> {
    fn slide(self, n: usize) -> Slide<T, A>;
}

impl<A: Clone, T: Iterator<Item=A>> SlideIterator<T, A> for T {
    fn slide(self, n: usize) -> Slide<T, A> {
        Slide::new(self, n)
    }
}

#[test]
fn test_slide() {
    let mut slide_iter = vec![1i8, 2, 3, 4, 5].into_iter().slide(3);
    assert_eq!(slide_iter.next().unwrap(), vec![1, 2, 3]);
    assert_eq!(slide_iter.next().unwrap(), vec![2, 3, 4]);
    assert_eq!(slide_iter.next().unwrap(), vec![3, 4, 5]);
    assert!(slide_iter.next().is_none());
}

#[test]
fn test_slide_equal_window() {
    let mut slide_iter = vec![1i8, 2, 3, 4, 5].into_iter().slide(5);
    assert_eq!(slide_iter.next().unwrap(), vec![1, 2, 3, 4, 5]);
    assert!(slide_iter.next().is_none());
}

#[test]
fn test_slide_zero_window() {
    let mut slide_iter = vec![1i8, 2, 3, 4, 5].into_iter().slide(0);
    assert!(slide_iter.next().is_none());
}

#[test]
fn test_slide_overlong_window() {
    let mut slide_iter = vec![1i8, 2, 3, 4, 5].into_iter().slide(7);
    assert!(slide_iter.next().is_none());
}
