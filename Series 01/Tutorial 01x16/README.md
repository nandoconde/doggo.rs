# [01x16] How to use a Struct in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=QM-6i3i2Nw0)

## Table of contents

- [\[01x16\] How to use a Struct in Rust](#01x16-how-to-use-a-struct-in-rust)
  - [Table of contents](#table-of-contents)
  - [Structs](#structs)
  - [Rust implementation](#rust-implementation)

## Structs

A user-defined collection of named fields, each with its own user-defined type.

- **Declaration:** *(also, definition)* the place in the code where the "shape" of
 the struct (name, order and type of fields) is defined.
- **Instances:** each object created using the struct as template is an isntance.

## Rust implementation

- **Declaration:** structs must be defined outside the main function
- **Naming:** by convention, struct names should begin with an uppercase letter
- **Types:** each field must have a known type at declaration
- **Strings:** if a field is going to contain a string of characters,
 it should use `String`, not `&str`; use `"x".to_string()` or `String::from("x")`.
