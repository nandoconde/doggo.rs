# [01x20] How to use Control Flow in Rust: For Loop

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=4z40dXzctbQ)

## Table of contents

- [\[01x20\] How to use Control Flow in Rust: For Loop](#01x20-how-to-use-control-flow-in-rust-for-loop)
  - [Table of contents](#table-of-contents)
  - ["For" loops](#for-loops)
    - [Scope of index variable](#scope-of-index-variable)
    - [Nested loops](#nested-loops)
    - [Naming loop variables](#naming-loop-variables)
    - ["For" in Rust](#for-in-rust)
  - [Ranges](#ranges)
    - [Ranges in Rust](#ranges-in-rust)

## "For" loops

In programming, a "for" loop statement refers to repeatedly performing the same tasks for a
 predefined number of times. The loop goes through each element in a container until all the
 elements have been consumed. The element is made available to the body of the loop for each round
 of iteration.

There are several tools associated with "for" loops:

- Ranges: the usual container is a collection of consecutive numbers, a [range](#ranges).
- `break` statements: stops the loop earlier.
- `continue` statements: skips to the following iteration of the loop.

The usual structure of a for loop expression is as follows:

```julia
for element in container_of_elements 
  # body

  # condition to skip iterations
  if (condition_skip)
    continue
  end

  # condition to break early
  if (condition_break)
    break
  end
end
```

### Scope of index variable

The variable `element` is created for the context of the "for" loop, and is not usually needed to
 declare it. However, it also lives only within the scope of the loop, and it is destroyed
 afterwards.

### Nested loops

A loop can contain another loop in its body. In this case, the variable of iteration of the outer
 loop can be accessed at any point by the inner loop, but will be cosntant across all the
 iterations of the inner loop. It will only change in the following iteration of the outer loop.

### Naming loop variables

The name of the loop variable can be any valid variable name, but it is usually drawn from a list
 of standard names. They usually follow a consecutive order so that the variable name indicates the
 level of nested "for" loop indentation.

- Letters after `i`:
  - `i`, `j`, `k`, `l`, `m`, `n`
  - Only up to six levels
  - Used in most general cases
- Letters after `m`:
  - `m`, `n`
  - Only up to two levels
  - Very used to iterate through matrices
- Letters after `p`:
  - `p`, `q`, `r`, `s`, `t`, `u`, `v`, `w`
  - Never more than five levels
  - Other starting points in this range:
    - `r`: only up to three levels, used for complex plane
    - `s`: only up to two levels, used in local coordinates
    - `u`: only up to three levels, used in local coordinates
- Letters after `x`:
  - `x`, `y`, `z`
  - Only up to three levels
  - Very used to iterate through coordinates

### "For" in Rust

- `while true` does not compile.
- Instead, there exists the keyword `loop`.
- The body is surrounded by brackets.
- The body is just as any other Rust content.

```rust
for i in container {
  // body

  // condition to skip iteration
  if condition_skip {
    continue;
  }

  // condition to exit early
  if condition_exit {
    break;
  }
}
```

## Ranges

A range is a collection of ordered numbers for which each element follows an increment/decrement
 relationship between the previous and the following. For example, a range of three elements
 between 1 and 3 (included) would be `1, 2, 3`.

Ranges are very useful in "for" loops because they allow to express in a compact way how many
 iterations are desired, and have access to the number of iteration within the body of the loop.

### Ranges in Rust

- Unit step: `init..last`
  - `init` is included in the range
  - `last` is not included in the range
  - `len(range(init, last))` is `last - init`
- Unit step (including upper bound): `init..=last`
  - `init` is included in the range
  - `last` is included in the range
  - `len(range(init, last))` is `last - init + 1`
- Reverse range:
  - `(init..last).rev()` does not include `last`
  - `(init..=last).rev()` includes `last`
  - Lengths are as in the non-reverse case
- Linear step:
  - `(init..top).step_by(step)` does not include `top`
  - `(init..=top).step_by(step)` includes `top`
  - `top - 1` (non-including case) or `top` (including case) is included in the range only if the
  last increment lies exactly there. Otherwise, the last element is
  `last = top - ((top - init) % step)` (including case). For non-including, the same formula
  applies but needs substitutint `top` with `top - 1`.
  - `length(init:step:top)` is `floor((top - init) / step) + 1`
