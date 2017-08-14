//! A sliding-window iterator library. Usable with any `Iterator` or `IntoIterator` with clonable
//! values.
//! Example:
//!
//! ```
//! # fn main() {
//! use iterslide::SlideIterator;
//! let v: Vec<i8> = vec![1, 2, 3, 4, 5];
//!
//! for window in v.slide(3) {
//!   // window is a VecDeque<i8>
//!   println!("{:?}", window);
//! }
//!
//! /*
//! Output:
//! [1, 2, 3]
//! [2, 3, 4]
//! [3, 4, 5]
//! */
//! # }

use std::collections::VecDeque;

/// Slide is a sliding-window iterator. The `next` method returns a `VecDeque` representing the
/// current window.
pub struct Slide<T: Iterator<Item=A>, A> {
    iter: T,
    n: usize,
    window: VecDeque<A>
}

impl<A: Clone, T: Iterator<Item=A>> Slide<T, A> {
    fn push_window(&mut self) -> bool {
        match self.iter.next() {
            Some(x) => {
                self.window.push_back(x);
                true
            },
            None => false,
        }
    }

    fn new(iter: T, n: usize) -> Slide<T, A> {
        Slide {
            iter: iter,
            n: n,
            window: VecDeque::with_capacity(n)
        }
    }
}

impl<A: Clone, T: Iterator<Item=A>> Iterator for Slide<T, A> {
    type Item = VecDeque<A>;

    fn next(&mut self) -> Option<VecDeque<A>> {
        if self.n == 0 {
            return None;
        }

        // Uninitialized case. Fill the first window
        if self.window.len() == 0 {
            while self.window.len() < self.n {
                if !self.push_window() {
                    return None;
                }
            }
        }
        // Otherwise, shift the window by 1
        else {
            self.window.pop_front();
            if !self.push_window() {
                return None;
            }
        }

        Some(self.window.clone())
    }
}

pub trait SlideIterator<T: Iterator<Item=A>, A> {
    /// Returns a sliding-window iterator
    fn slide(self, n: usize) -> Slide<T, A>;
}

impl<A: Clone, T: IntoIterator<Item=A>> SlideIterator<T::IntoIter, A> for T {
    fn slide(self, n: usize) -> Slide<T::IntoIter, A> {
        Slide::new(self.into_iter(), n)
    }
}

#[test]
fn test_slide() {
    // NB: Although the window is a VecDeque, we can still compare it to slices, since VecDeque
    // implements PartialEq for a ton of types
    let mut slide_iter = vec![1i8, 2, 3, 4, 5].slide(3);
    assert_eq!(slide_iter.next().unwrap(), &[1, 2, 3]);
    assert_eq!(slide_iter.next().unwrap(), &[2, 3, 4]);
    assert_eq!(slide_iter.next().unwrap(), &[3, 4, 5]);
    assert!(slide_iter.next().is_none());
}

#[test]
fn test_slide_equal_window() {
    let mut slide_iter = vec![1i8, 2, 3, 4, 5].slide(5);
    assert_eq!(slide_iter.next().unwrap(), &[1, 2, 3, 4, 5]);
    assert!(slide_iter.next().is_none());
}

#[test]
fn test_slide_zero_window() {
    let mut slide_iter = vec![1i8, 2, 3, 4, 5].slide(0);
    assert!(slide_iter.next().is_none());
}

#[test]
fn test_slide_overlong_window() {
    let mut slide_iter = vec![1i8, 2, 3, 4, 5].slide(7);
    assert!(slide_iter.next().is_none());
}
