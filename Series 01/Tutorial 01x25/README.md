# [01x25] What does "References and Borrowing" mean in Rust?

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=6I0cKZQkLVU)

## Table of contents

- [\[01x25\] What does "References and Borrowing" mean in Rust?](#01x25-what-does-references-and-borrowing-mean-in-rust)
  - [Table of contents](#table-of-contents)
  - [Reference](#reference)
  - [Borrowing](#borrowing)

## Reference

A reference is a link to a value that does not grant ownership, just "borrowship".

A reference is created using `&` in the type declaration, e.g., `&String`.

## Borrowing

Using a reference to a value without taking ownership of it.

The scope that borrows the value is known as the *borrower*.
