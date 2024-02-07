# [01x10] How to use Different Types of Numbers in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=YkvLVp0g8EE)

## Table of contents

- [\[01x10\] How to use Different Types of Numbers in Rust](#01x10-how-to-use-different-types-of-numbers-in-rust)
  - [Table of contents](#table-of-contents)
  - [Types of numbers](#types-of-numbers)
  - [Precision](#precision)
  - [Rust number types](#rust-number-types)
    - [Integers](#integers)
    - [Floating points](#floating-points)
  - [Precision printing](#precision-printing)
  - [Casting](#casting)

## Types of numbers

- Integers: 'whole' numebrs, without decimal part, used mostly for counting number of things or
   positions.
- Floats: floating point numbers, used to represent numbers with decimals in a finite
   representation system (no infinite memory available means it is impossible to represent every
   number in the real line).
- [More types in the Rust documentation: doc.rust-lang.org](doc.rust-lang.org)

## Precision

- **32-bit**: uses 32 bits to store information
  - Faster
  - Limited precision
- **64-bit**: uses 64 bits to store information
  - Slower (still usually fast)
  - Extended precision

## Rust number types

A particular type of number is used by adding a *type declaration* after the variable name:

```rust
// Declare a 64-bit signed integer
let b: i64 = 3
```

### Integers

- `i32`: 32-bit signed integer **(default)**.
- `i64`: 64-bit signed integer.
- Digit separator: underscore `_` can be used when entering an integer number to visualize it more clearly:

  ```rust
  // To clearly see a million
  let x = 1_000_000;
  ```

### Floating points

- `f32`: 32-bit float.
- `f64`: 64-bit float **(default)**.

## Precision printing

- Decimal part: `{:.X}` prints `X` decimals after the decimal separator.

## Casting

- The type of function results can be constrained to the desired data types:

  ```rust
  let d = f32::sqrt(4.0);
  ```
