# [01x24] What does "Ownership" mean in Rust?

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=QrPYzCgS31Q)

## Table of contents

- [\[01x24\] What does "Ownership" mean in Rust?](#01x24-what-does-ownership-mean-in-rust)
  - [Table of contents](#table-of-contents)
  - [Ownership](#ownership)
  - [Example](#example)

## Ownership

"Ownership" refers to a set of rules that govern how Rust manages memory. They
 allow the Rust compiler to make guarantees about allocations and memory safety
 without needing a Garbage Collector (GC).

These are the rules of ownership:

- Each value has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

## Example

1. Create a value
2. Use it in the main scope
3. Pass it to a function
4. Go back to the original function
5. Try to use it
6. Error: the value was dropped when exiting the function call
