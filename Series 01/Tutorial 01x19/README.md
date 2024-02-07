# [01x19] How to use Control Flow in Rust: Infinite Loop

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=WdpUpwziNec)

## Table of contents

- [\[01x19\] How to use Control Flow in Rust: Infinite Loop](#01x19-how-to-use-control-flow-in-rust-infinite-loop)
  - [Table of contents](#table-of-contents)
  - [Infinite loops](#infinite-loops)
    - [Rust](#rust)
    - [Python](#python)
    - [Julia](#julia)

## Infinite loops

In programming, an infinite loop statement refers to repeatedly performing the same tasks without
 constantly checking for a condition to stop. The only way to stop doing these tasks is to
 explicitly tell the computer to exit the loop.

The usual structure of an infinite loop expression is as follows:

```C
while (true) {
  // body

  // condition
  if (condition) {
    break;
  };
}
```

This does not need to check any condition, because the condition is always true. Depending on the
 programming language, the way this structure is expressed may vary.

### Rust

- `while true` does not compile.
- Instead, there exists the keyword `loop`.
- The body is surrounded by brackets.
- The body is just as any other Rust content.

```rust
loop {
  // body

  // condition
  if condition {
    break;
  }
}
```

### Python

- The body is indented one level.
- The body is just as any other Python content.
- It must return to normal indentation level to leave the `while` body.

```python
while True:
  # body, necessarily indented one level

  # condition
  if condition:
    break
```

### Julia

- Condition does not need parentheses and must evaluate to a boolean expression.
- The body begins after the conditional expression.
- The body is just as any other Julia content.
- The body ends with an `end` statement.

```julia
while true
  # body

  # condition
  if condition
    break
  end
end
```
