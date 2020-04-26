rust-iterslide
===============

[![Version](https://img.shields.io/crates/v/iterslide.svg)](https://crates.io/crates/iterslide)
[![Docs](https://docs.rs/iterslide/badge.svg)](https://docs.rs/iterslide)
[![Build Status](https://travis-ci.org/rozbb/rust-iterslide.svg?branch=master)](https://travis-ci.org/rozbb/rust-iterslide)

This package implements "sliding window" functionality for any iterator over a `Clone`able item.

Example
-------

```rust
use iterslide::SlideIterator;

fn main() {
    let v: Vec<i8> = vec![1, 2, 3, 4, 5];

    for window in v.slide(3) {
        // window is a VecDeque<i8>
        println!("{:?}", window);
    }
}
```

Output:

```
[1, 2, 3]
[2, 3, 4]
[3, 4, 5]
```

Alternatives
------------

* Rust's [`windows`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows) method for slices does what you would expect. This does no allocation. However, this is only implemented for slices.

* itertools' [`tuple_windows`](https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.tuple_windows) method returns a sliding window iterator over tuples instead of `VecDeques`. This saves a `VecDeque` allocation in construction of the iterator. However, `tuple_windows` is limited to window sizes of at most 4.

License
-------

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
 * MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
