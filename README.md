# spans

This crate allows you to split an iterator into contiguous spans.

Import the `Spans` trait to extend `Iterator`:

```rust
use spans::Spans;
```

Now you can use `Spans::spans_by_key` to split an iterator into contiguous spans:

```rust
let vec = vec![1, 2, 5, 6, 7, 11, 13, 14, 15];
let mut spans = vec.iter().spans_by_key(|&&x| x, |a, b| a + 1 == b);

while let Some(span) = spans.next() {
    println!("span = {:?}", span.collect::<Vec<_>>());
}
```

The code above splits the vector into spans where each item is 1 larger than the proceeding item.
The following text is printed:

```text
span = [1, 2]
span = [5, 6, 7]
span = [11]
span = [13, 14, 15]
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

* * *

Many thanks to [Matt Brubeck](https://users.rust-lang.org/u/mbrubeck) for helping me so generously on [the Rust users forum](https://users.rust-lang.org/t/split-iterator-into-iterator-of-iterators/54281).
