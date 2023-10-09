# [01x02] How to install Rust onto my computer

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=QSwEBgyX5ds)

## Table of contents

[1. Installing `rustup`](#installing-rustup)
[2. Check the Rust installation](#check-the-rust-installation)

## Installing `rustup`

This requires an active Internet connection.

### Linux/macOS

1. Open a terminal
2. Run `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
3. Continue with default Rust installation pressing `1`.

### Windows

1. Enter [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Download the `rustup` installer (most likely 64-bit)
3. Execute the `rustup-int.exe` installer
   1. If prompted to install Visual C++ through Visual Studio installer,
   enter `1`.
   2. Continue as default throughout this sub-installation.
4. Continue with default Rust installation pressing `1`.

## Check the Rust installation

1. Open a terminal
2. Enter `rustc --version`
3. Something like `rustc 1.72.0 (5680fa18f 2023-08-23)` should appear, but
   numbers may vary.
