rust-iterslide
===============

This package implements a "sliding window" iterator.

Sample usage:

```rust
extern crate iterslide;

use iterslide::SlideIterator;

fn main() {
  for window in vec![1i, 2, 3, 4, 5].into_iter().slide(3) {
    println!("{}", window);
  }
}
```

Contributors
----

[@doomrobo](https://github.com/doomrobo) updated the code to run on the
latest Rust version. Thanks, @doomrobo! :)
