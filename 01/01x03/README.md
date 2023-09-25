# [01x03] Hello, World! | How to Write, Compile and Run Code using Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=ztRX_q_cXUI)

## Table of contents

[1. Create a Rust source code file](#create-a-rust-source-code-file)
[2. Compile the code to a native binary](#compile-the-source-code-to-a-native-binary)
[3. General workflow for Rust](#general-workflow-for-rust)

## Create a Rust source code file

Write the first Rust program, which typically is printing `Hello, World!` in an
open terminal.

- `//` begins a comment
- `fn` defines a function
- `main` is the usual name for the function acting as entry point to the program
- `()` defines no arguments to the function
- `println` prints text to the standard output (console)
- `!` indicates that it is a macro
- `"Hello, World!"` is the string of characters that will be printed
- `{}` delimit the body of the function

The source file can be found at [`hello.rs`](hello.rs).

## Compile the source code to a native binary

1. Open a terminal and navigate to the folder where `hello.rs` is located.
2. Execute `rustc hello.rs`
   1. `rustc` executes the Rust compiler
   2. `hello.rs` tells the compiller to compile the source code in that file
3. Depending on the operating system, different things happen:
   1. *Windows:* there are two new files in the current folder, `hello.pdb` and
    `hello.exe`. This last file is the executable.
   2. *Linux:* there is just a new binary file in the folder, called `hello`.
4. The executable file created by `rustc` is portable across computers with
   the same architecture (among all Windows for `hello.exe`, or among all the
   same Linux distros for `hello`). The other computers do not need to have
   Rust installed on them.

## General workflow for Rust

1. Write source code in one or several files
2. Compile the source code with Rust compiler `rustc`
3. Run the executable/binary file generated
