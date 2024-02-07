# [01x23] How to use a Method in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=pFI16zgetLU)

## Table of contents

- [\[01x23\] How to use a Method in Rust](#01x23-how-to-use-a-method-in-rust)
  - [Table of contents](#table-of-contents)
  - [Methods](#methods)
  - [Rust implementation](#rust-implementation)

## Methods

A method is a function defined in the context of a struct. They are associated to specific structs,
 and usually perform operations with using the fields inside.

## Rust implementation

- `impl` opens a scope to define methods
- Methods are defined as functions
- `&self` must be the first parameter to each method
- `self` refers, within the body of the method, to the instance of the struct over which the function will operate
- Fields are accessed as usual, doing `self.a`, etc.

```rust
// struct definition
struct MyStruct {
    a: T,
    b: T,
    // etc
}

// method definitions
impl MyStruct {
    fn method_1(&self) -> T {
        // operations with the internal fields
        (self.a + self.b) * self.a
    }
}
```
