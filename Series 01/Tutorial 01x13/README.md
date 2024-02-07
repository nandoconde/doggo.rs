# [01x13] How to use Vectors in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=gqeuTxM945c)

## Table of contents

- [\[01x13\] How to use Vectors in Rust](#01x13-how-to-use-vectors-in-rust)
  - [Table of contents](#table-of-contents)
  - [Vector](#vector)

## Vector

In Rust, vectors are arrays of mutable length.
The fact that they exist does not mean that they should be used extensively, as they hinder performance.
A *deque* may be used instead.

- **Declaration**
  `let mut v = vec![...];`
- **Adding elements**
  - *Appending*: `v.push(100)`
  - *Prepending*: `v.insert(100, 0)`, very inefficient (with *O(n)*).
  - *Insert at x*: `v.insert(100, i)`, shifts elements to the right.
- **Removing elements**
  - *Last element*: `v.pop()`
  - *First element*: `v.remove(0)`, very inefficient (with *O(n)*).
  - *Remove at*: `v.remove(i)`, shifts elements to the left.
  - *Remove at (efficient)*: `v.swap_remove(i)`. It changes element `i` for the last element in the vector and `pop`s last.
