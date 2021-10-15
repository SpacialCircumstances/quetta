# quetta

(from the Quenya word for "word") is a library providing simple
immutable strings in Rust.
Essentially, it is a wrapper around `Arc<str>`, but with support for slicing and compatibility features
with `&str`.

The primary type provided is `quetta::Text`, which is either a owned string (immutable and refcounted atomically) or a slice into one.

## Motivation

Strings in Rust are relatively cumbersome to use (compared to high-level languages like Java, C#, OCaml etc.).
For dealing with strings, there are two common choices:

1) Use an owned `String`
   - Easy to pass around
   - Requires cloning frequently, which can be inefficient
   - Is mutable
   - Substrings/slices create either a borrowed `&str` or cause copying

2) Use a string slice `&str`
   - Requires frequent lifetime annotations
   - Requires keeping the owning `String` around
   - Can be easily sliced

`quetta::Text` aims to make dealing with this easier, especially for applications like GUI apps and compilers that often have to deal and pass around text.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
quetta = "0.1.0"
```

## Example

```rust
use quetta::Text;

let t: Text = Text::new("a.b.c");
let s1: Text = t.slice(0, 2);
assert_eq!("a.", s1.as_str());
```

For more examples, see the [documentation](https://docs.rs/tinyvec/) or take a look at the [code](https://github.com/SpacialCircumstances/quetta).
