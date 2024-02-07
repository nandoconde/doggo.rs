# [01x22] How to use a Function in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=fEJq9YUf2Z4)

## Table of contents

- [\[01x22\] How to use a Function in Rust](#01x22-how-to-use-a-function-in-rust)
  - [Table of contents](#table-of-contents)
  - [High-level programming](#high-level-programming)
    - [Abstraction](#abstraction)
    - [Decomposition](#decomposition)
  - [Functions](#functions)
    - [Declaration](#declaration)
    - [Call](#call)
    - [Overloading](#overloading)
    - [Rust implementation](#rust-implementation)

## High-level programming

When viewed from a high level perspective, functions serve two purposes: abstraction and decomposition.

### Abstraction

Abstraction means hiding all the code performing a task behind an alias (a function). This means
 that the person coding the program:

- does not need to know how to implement every line of code in that task,
- needs to know what to pass (parameters) as the input to the function for it to take care of the task,
- needs to know what to expect (results) as the output to the function for, and
- needs to know (with varying level of detail) what the function does to transform the inputs to the outputs.

### Decomposition

Decomposition means breaking a large computation/program into several blocks. Each block performs
 only related tasks, possibly unitary, and can be further subdivided if too complex. This favors:

- code writing,
- code debugging,
- code maintenance, and
- understanding.

## Functions

### Declaration

Functions, in any programming language, are defined by:

- function keyword
- name of the function
- number of parameters
- type of the parameters
- return type(s)
- body
- return statement

A line containing the first five items is usually called the **header** or the **signature** of
 the function, and identifies univocally the function. This means that the function will only work
 with that many arguments, and of exactly that type.

### Call

When a function is called from a piece of code, the objects supplied as parameters are called

### Overloading

Function overloading refers to having two function definitions with the same name. It is not
 allowed in Rust. For each function, a new name must be defined. The same behavior as overloading
 is reached in Rust by using *traits*.

### Rust implementation

- `fn`: function keyword
- `my_function`: name of the function:
  - any valid variable name
  - usually in `snake_case`
- `(x: R, y: S)`: parameters and types:
  - enclosed in parentheses
  - with known types declared after each parameters name
- `-> T`: return type
- `{...}`: body of the function
  - as many statements as needed
  - good programming practices limit the body to ~100 lines max.
  - the return statement is the last line of the function body

```rust
fn my_function(arg1: X, arg2: Y) -> T {
    // body

    // return statement
    arg1 * arg2
}
```
