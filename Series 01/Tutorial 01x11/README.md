# [01x11] How to use Characters, String Literals and Strings in Rust

Source: [doggo dot rs (YouTube)](https://www.youtube.com/watch?v=cmpeozX-yoI)

## Table of contents

- [\[01x11\] How to use Characters, String Literals and Strings in Rust](#01x11-how-to-use-characters-string-literals-and-strings-in-rust)
  - [Table of contents](#table-of-contents)
  - [Characters](#characters)
  - [String literals](#string-literals)
  - [Strings](#strings)
  - [String utilities](#string-utilities)

## Characters

- `char` is a type for representing a single character.
- Defined as a *Unicode scalar value*.
- `'a'` is a `char` literal and can be assigned directly to a variable.

## String literals

- `&str` is a type for representing an **immutable** string of characters.
- 'Literal' means that it must be entered in the code "as is", not as the result of computations.
- `"this is a string literal"` is a `&str` literal and can be assigned directly to a variable.

## Strings

- `String` is a type for representing a **mutable** string of characters.

## String utilities

- Special characters:
  - Newline: `\n`
  - Tab: `\t`
  - Quotation marks: `\"`
  - Single quotation mark: `\'`
  - Backslash: `\\`
  - Bytes by HEX values: `\xFF`
  - Unicode points: `\u{211D}`
  - Multiline non-break: `\`
- Multiline strings: entered byu just adding several lines within a string type.
- Raw strings: strings where none of the characters are escaped.
- String concatenation: only mutable strings can be concatenated.
  - `&str` needs to be converted first to a `String` with `.to_string()``.`
  - Concatenation performed with `+`.
