# [01x12] How to use Arrays in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=aOBdXPNboSg)

## Table of contents

- [\[01x12\] How to use Arrays in Rust](#01x12-how-to-use-arrays-in-rust)
  - [Table of contents](#table-of-contents)
  - [Data structures](#data-structures)
  - [Arrays](#arrays)
  - [N-dimensional arrays](#n-dimensional-arrays)

## Data structures

Aggregated collection of variables and data in a single object.

In Rust, all data structures are called **compund types**.

## Arrays

Data structure comprising an ordered collection of elements of the same type.

In Rust:

- **length**:
  - *fixed-length arrays* only
  - length found through `v.len()`
- **mutability**:
  - *mutable arrays*: the elements within the array may change
    - declared with `let mut v = [...];`
    - element access with `v[i]`
    - mutated with `v[i] = x;`
  - *immutable arrays*: the elements within the array are fixed at execution or compilation
    - declared with `let v = [...];`
    - element access with `v[i]`
    - mutation errors at compilation time
- **indexing**: 0-indexed
- **methods**:
  - *reverse*: `.reverse()`
  - *iterator*: `.iter()`
    - *sum*: `.iter().sum()`, needs type declaration
  - *sorting*: `.sort()`, by value

## N-dimensional arrays

In Rust, no constructor for 2-dimensional or N-dimensional arrays. Use array of arrays instead.

- *construction*: `let v = [[...], [...], ...];`
- *access*: `v[i][j]`, j-th element of i-th array in constructor.
