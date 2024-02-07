# [01x18] How to use Control Flow in Rust: While Loop

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=jYRsmmwVWVU)

## Table of contents

- [\[01x18\] How to use Control Flow in Rust: While Loop](#01x18-how-to-use-control-flow-in-rust-while-loop)
  - [Table of contents](#table-of-contents)
  - [While loops](#while-loops)
    - [Rust](#rust)
    - [Python](#python)
    - [Julia](#julia)

## While loops

In programming, a `while` statement refers to repeatedly performing the same tasks under the
 condition that a requirement is met. As soon as the requirement/condition is not met anymore, the
 computer stops performing the tasks. The tasks performed within the body of the statement are able
 to modify the conditions so that the program may eventually stop.

The usual structure of an `while` expression is as follows:

```rust
while (condition) {
  // body
}
```

Depending on the programming language, the way this structure is expressed may vary.

### Rust

- Condition does not need parentheses and must evaluate to a boolean expression.
- The body is surrounded by brackets.
- The body is just as any other Rust content.

```rust
while condition {
  // body
}
```

### Python

- Condition does not need parentheses and must evaluate to a boolean expression.
- The body is indented one level.
- The body is just as any other Python content.
- It must return to normal indentation level to leave the `while` body.

```python
while condition:
  # body, necessarily indented one level

```

### Julia

- Condition does not need parentheses and must evaluate to a boolean expression.
- The body begins after the conditional expression.
- The body is just as any other Julia content.
- The body ends with an `end` statement.

```julia
while condition
  # body
end
```
