rust-iterslide
===============

[![Build Status](https://travis-ci.org/purpliminal/rust-iterslide.svg?branch=master)](https://travis-ci.org/purpliminal/rust-iterslide)

This package implements a "sliding window" iterator.

Sample usage:

```rust
extern crate iterslide;

use iterslide::SlideIterator;

fn main() {
    let x: Vec<i8> = vec![1, 2, 3, 4, 5];
    
    for window in x.into_iter().slide(3) {
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

Contributors
----

[@doomrobo](https://github.com/doomrobo) updated the code to run on the
latest Rust version. Thanks, @doomrobo! :)
