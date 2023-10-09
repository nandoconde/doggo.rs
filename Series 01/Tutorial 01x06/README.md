# [01x06] How to use Rust in VS Code

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=jvnZr7bJUfU)

## Table of contents

[1. Customize VS Code](#customize-vs-code)
[2. Rust + VS Code](#rust--vs-code)
[3. Check the Rust installation](#check-everything-is-running)

## Customize VS Code

1. If prompted, select a theme for VS Code.
   1. Darker themes strain less the eyes.
   2. More themes can be installed later through the Extensions Markeplace.
2. Font size can be changed through **Ctrl + `+`** and **Ctrl + `-`**.

## Rust + VS Code

Requires an active Internet connection.

1. On the leftmost panel, click on "Extensions" (four blocks, one of them detached).
2. On the textbox, enter "rust".
3. For the result "rust-analyzer", press the blue button that reads "Install".
4. Now, enter on the textbox "code runner".
5. For the result with more than 21.6 million installs, press "Install".
6. WSL only:
   1. WSL extension by Microsoft can be installed searching for "WSL" and pressing "Install" on the extension whose developer is "Microsoft"

### Check everything is running

1. On the leftmost panel, click on "Explorer" (a sheet of paper).
2. Create the same `hello.rs` as in [Lesson 01x03](../01x03/hello.rs)
3. On the rightmost top there is a triangle, whose text while hovered reads "Run". Press it.
4. The lowest part of the screen should change. If everything is ok, it shows something like:

```bash
[Running] cd "/home/fcpv/nandoconde/doggo.rs/01/01x06/" && rustc hello.rs && "/home/fcpv/nandoconde/doggo.rs/01/01x06/"hello
Hello, World!

[Done] exited with code=0 in 0.857 seconds
```
