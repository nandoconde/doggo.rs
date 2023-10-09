# [01x07] How to perform basic math in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=xOZ1h7gjeC4)

## Table of contents

[1. Lesson environment](#lesson-environment)
[2. Comments](#comments)
[3. Integer math operations](#integer-math-operations)

## Lesson environment

Create a file for the soruce code `math.rs`. Any name is valid as long as it has the `.rs` extension

## Comments

1. Comments are notes interpsersed in the code. They may help oneself or others:
   1. Understanding what the code does
   2. Understanding why it was coded that way
   3. Suggesting alternatives in case it does not work
   4. Keeping a list of tasks related to that code (e.g.: comments beginning with `TODO`)
2. Comments in code begin with `//` . They can be placed anywhere:
   1. Beginning of a line
   2. Midst of a line
3. Multi-line comments are just several lines together acting as a signle comment.
4. VS Code default shortcut for toggling comments in a line is **Ctrl + `/`**, although it may vary among regions and OS.

## Integer math operations

1. All integer operations return integers
2. Non-integer results of division still return the integer part of the division
3. Basic math operators:
   1. Addition: `+`
   2. Subtraction: `-`
   3. Multiplication: `*`
   4. Division: `/`
   5. Modulo: `%`
4. Using built-in functions:
   1. Exponentiation: `i32::pow(base, exponent)`
