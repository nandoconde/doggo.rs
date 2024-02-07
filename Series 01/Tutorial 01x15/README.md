# [01x15] How to use a HashMap in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=tHN-cN6a494)

## Table of contents

- [\[01x15\] How to use a HashMap in Rust](#01x15-how-to-use-a-hashmap-in-rust)
  - [Table of contents](#table-of-contents)
  - [Hash](#hash)
  - [Dictionaries | Hash Tables](#dictionaries--hash-tables)
    - [Definition](#definition)
    - [Rust](#rust)

## Hash

Mathematical operation over software objects that assings them an *almost* univocal value
 among its class. The value is usually represented as a hex-string or a 256-bit unsigned
 integer, but it may have different lengths depending on the algorithm used to hash.

Not all software objects have a predefined manner to perform a hash operation on them, and
 sometimes it must be manually defined.

## Dictionaries | Hash Tables

### Definition

Data structures holding values in an ordered, addressable manner: instead of using a
 continuous range of integer values (indices, such as in vectors, tuples or arrays) it
 uses other objects instead. These objects can be:

- strings (thus the name 'dictionary')
- numbers
- any other hashable object

The insert/remove performance remains constant regarless of the size, not as in arrays.

### Rust

Rust implements Hash Tables using `HashMap`.

For `HashMap`:

- all keys in the `HashMap` must have the same type,
- all values in the `HashMap` must have the same type,
- entries are stored unordered, and
- `println!` can display it with `{:#?}`.

Usual operations with `HashMap` are:

- **Creation:**
  - *Immutable:*`let mut x = HashMap::new();`
  - *Mutable:*`let mut x = HashMap::new();`
- **Insertion:** `x.insert(key, value);`
- **Deletion:** `x.remove(key);`
- **Printing:** `println!("{:#?}", x);`
