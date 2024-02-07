# [01x27] How to use Cargo in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=OywkpRNK1lk)

## Table of contents

- [\[01x27\] How to use Cargo in Rust](#01x27-how-to-use-cargo-in-rust)
  - [Table of contents](#table-of-contents)
  - [Cargo](#cargo)
  - [Creating a Rust project with Cargo](#creating-a-rust-project-with-cargo)
    - [Manifest file - `Cargo.toml`](#manifest-file---cargotoml)
    - [Entry point - `src/main.rs`](#entry-point---srcmainrs)
  - [Building a project with Cargo](#building-a-project-with-cargo)
  - [Running a project with Cargo](#running-a-project-with-cargo)

## Cargo

Cargo is Rust's build system and package manager.

## Creating a Rust project with Cargo

A Cargo project should be used to develop Rust code. To create a Cargo project named `hello_cargo`,
 the following command must be run in the terminal from the parent folder:

```bash
cargo new hello_cargo
```

It automatically creates a child folder `hello_cargo` with:

- Manifest file: `Cargo.toml`
- Source code folder: `src/`
- Program entry point: `src/main.rs`, with boilerplate code for the entry point.
- `.gitignore`: if a Git account is linked to the Cargo installation, a `.gitignore` is created automatically.

### Manifest file - `Cargo.toml`

The manifest file, written in [TOML](https://toml.io/en/), contains metadata needed to compile a
 package created with Cargo.
It consists of several sections and entries (see [spec.](https://doc.rust-lang.org/cargo/reference/manifest.html)
 for more details)

### Entry point - `src/main.rs`

The function `main` within the `src/main.rs` file is the first function called when executing a
 project compiled with Cargo.

## Building a project with Cargo

From a terminal session at the Cargo project folder:

```bash
cargo build
```

This creates:

- Locked manifest: `Cargo.lock`, a Cargo manifest with versions locked to those used to build the project.
- Targeted build: a folder with the binaries and metadata generated for the targeted build platform.

## Running a project with Cargo

From a terminal session at the Cargo project folder:

```bash
cargo run
```
