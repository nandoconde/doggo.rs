# [01x17] How to use Control Flow in Rust: if Expression (if...else if...else)

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=JO5tVoSbFn0)

## Table of contents

- [\[01x17\] How to use Control Flow in Rust: if Expression (if...else if...else)](#01x17-how-to-use-control-flow-in-rust-if-expression-ifelse-ifelse)
  - [Table of contents](#table-of-contents)
  - [Control flow](#control-flow)
  - [`if` conditional statement](#if-conditional-statement)
    - [Rust](#rust)
    - [Python](#python)
    - [Julia](#julia)

## Control flow

In computer science, *control flow* refers to the ways a program can be commanded to execute one
task or the other depending on some conditions, and for a number of times.

## `if` conditional statement

In programming, an `if` statement refers to performing tasks under the condition that a requirement is met.
The usual structure of an `if` expression is as follows:

```rust
if (condition) {
  // body
}
```

Depending on the programming language, the way this is expressed may vary.

### Rust

- Condition does not need parentheses and must evaluate to a boolean expression.
- The body is surrounded by brackets.
- The body is just as any other Rust content.

```rust
if condition {
  // body
}
```

### Python

- Condition does not need parentheses and must evaluate to a boolean expression.
- The body is indented one level.
- The body is just as any other Python content.
- It must return to normal indentation level to leave the `if` body.

```python
if condition:
  # body, necessarily indented one level

```

### Julia

- Condition does not need parentheses and must evaluate to a boolean expression.
- The body begins after the conditional expression.
- The body is just as any other Julia content.
- The body ends with an `end` statement.

```julia
if condition
  # body
end
```
