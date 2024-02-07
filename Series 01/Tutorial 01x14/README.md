# [01x14] How to use Tuples in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=23MAY6pBAQA)

## Table of contents

- [\[01x14\] How to use Tuples in Rust](#01x14-how-to-use-tuples-in-rust)
  - [Table of contents](#table-of-contents)
  - [Tuple](#tuple)

## Tuple

A tuple is an ordered collection of elements whose elements may have heterogeneous types.

- **Length**
  - *fixed-length*
  - length found through `t.len()`
- **Mutability**
  - *mutable tuples*
    - declared with `let mut mt = ('a', 1, "t");`
    - accessed with `mt.0;`
    - mutated with `mt.0 = 't';`
  - *immutable tuples*
    - declared with `let it = ('a', 1, "t");`
    - accessed with `mt.2;`
    - mutation errors at compilation time
  - *indexing*: 0-indexed
- **Destructuring assignment**: `let (c, n, s) = mt;`
