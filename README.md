# lodash-guru

**Lodash-style utility functions for Rust.** 25+ battle-tested helpers for slices and collections — `chunk`, `group_by`, `uniq`, `shuffle`, `partition`, `intersection` and more, with zero dependencies.

Formerly published on crates.io as `lodashguru`.

For developers coming from JavaScript/TypeScript who miss the familiar Lodash vocabulary, and for Rust developers who want one obvious place for everyday collection helpers.

## Usage

```rust
use lodash_guru::{chunk, group_by, shuffle, uniq};

fn main() {
    let pages = chunk(&[1, 2, 3, 4, 5], 2);
    assert_eq!(pages, vec![vec![1, 2], vec![3, 4], vec![5]]);

    let by_remainder = group_by(&[1, 2, 3, 4], |n| n % 2);

    let unique = uniq(&[1, 1, 2, 3, 3]);
    assert_eq!(unique, vec![1, 2, 3]);

    let mixed = shuffle(&[1, 2, 3, 4, 5]);
    println!("{mixed:?}");
}
```

## Functions

`chunk` · `compact` · `difference` · `drop_` · `drop_right` · `first` · `last` · `nth` · `take` · `flatten` · `uniq` · `union` · `intersection` · `xor` · `without` · `pull` · `group_by` · `partition` · `reject` · `includes` · `shuffle` · `sample` · `sample_size`

All functions are pure (except `pull`, which mutates in place), allocation-friendly and fully covered by tests.

## License

Licensed under either of MIT or Apache-2.0, at your option.

Name is a tribute to [Lodash](https://lodash.com); this crate has no code shared with it and is an independent implementation for Rust.
