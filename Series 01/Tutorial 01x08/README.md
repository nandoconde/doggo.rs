# [01x08] How to use Booleans in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=njUMvPAaJIc)

## Table of contents

- [\[01x08\] How to use Booleans in Rust](#01x08-how-to-use-booleans-in-rust)
  - [Table of contents](#table-of-contents)
  - [Lesson environment](#lesson-environment)
  - [Boilerplate code](#boilerplate-code)
  - [Boolean](#boolean)
  - [Comparison operators](#comparison-operators)
  - [Boolean operators](#boolean-operators)

## Lesson environment

Create a file called `bool.rs`. Again, any file with the `.rs` extension is good enough.

## Boilerplate code

Code needed as a bare minimum to start coding other things. Here in Rust, the boilerplate code
 for the entry point of the program is:

```rust
fn main() {
    // your code here...
}
```

## Boolean

Indicator of whether something is true or false.
 It can be introduced by the programmer or as the result of some operations in the code.
 `true` and `false` are the nominal values of booleans.

## Comparison operators

The typical comparison operators are available in Rust. Their precedence is higher than the boolean operators,
 but caution and parentheses are advised to make the code clearer and less prone to errors.

These operators are:

- Less than: `<`
- Less than or equal to: `<=`
- Greater than: `>=`
- Greater than or equal to: `>=`
- Equal to: `==`
- Not equal to: `!=`

## Boolean operators

They are evaluated lazily, i.e.: if only the first evaluation is needed to determine the output,
 the second is not performed.

- AND: `&&`
- OR: `||`
