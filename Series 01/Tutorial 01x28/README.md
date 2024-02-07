# [01x28] How to use the rust-analyzer Extension in VS Code

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=ox2UW4VSxRg)

## Table of contents

- [\[01x28\] How to use the rust-analyzer Extension in VS Code](#01x28-how-to-use-the-rust-analyzer-extension-in-vs-code)
  - [Table of contents](#table-of-contents)
  - [Rust analyzer](#rust-analyzer)
  - [VS Code](#vs-code)

## Rust analyzer

The Rust analyzer extension is a Language Server that provides coding tools specific to the Rust
 programming language.

The Rust analyzer extension assumes that a Cargo project is associated to every Rust project, and
 needs to locate it to work. It provides:

- type hints,
- inline docs,
- suggestions and tab completion (functions, type hints, semicolon), and
- automatic boilerplate.

## VS Code

The Rust analyzer needs to detect a single Cargo project in the available folder. Usually, it is a good practive to have only one active Cargo project in the VS Code Workspace.
