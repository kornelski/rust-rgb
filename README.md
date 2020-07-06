# `struct RGB` for [Rust](https://www.rust-lang.org)  [![crate](https://img.shields.io/crates/v/rgb.svg)](https://lib.rs/crates/rgb)

Operating on pixels as weakly-typed vectors of `u8` is error-prone and inconvenient. It's better to use vectors of pixel structs. However, Rust is so strongly typed that *your* RGB pixel struct is not compatible with *my* RGB pixel struct. So let's all use mine :P

[![xkcd standards](https://imgs.xkcd.com/comics/standards.png)](https://xkcd.com/927/)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rgb = "0.8"
```

## Usage

### `RGB` and `RGBA` structs

The structs implement common Rust traits and a few convenience functions, e.g. `map` that repeats an operation on every subpixel:

```rust
use rgb::*; // Laziest way to use traits which add extra methods to the structs

let px = RGB {
    r:255_u8,
    g:0,
    b:255,
};
let inverted = px.map(|ch| 255 - ch);

println!("{}", inverted); // Display: rgb(0,255,0)
assert_eq!(RGB8::new(0, 255, 0), inverted);
```

### Byte slices to pixel slices

For interoperability with functions operating on generic arrays of bytes there are functions for safe casting to and from pixel slices.

```rust
let raw = vec![0u8; width*height*3];
let pixels: &[RGB8] = raw.as_rgb(); /// Safe casts without copying
let raw_again = pixels.as_bytes();
```

Note: if you get an error about "no method named `as_bytes` found", add `use rgb::ComponentBytes`. If you're using a custom component type (`RGB<CustomType>`), implement `rgb::Pod` (plain old data) and `rgb::Zeroable` trait for the component (these traits are from [`bytemuck`](//lib.rs/bytemuck) crate).

----

## About colorspaces

*Correct* color management is a complex problem, and this crate aims to be the lowest common denominator, so it's intentionally agnostic about it.

However, this library supports any subpixel type for `RGB<T>`, and `RGBA<RGBType, AlphaType>`, so you can use them with a newtype, e.g.:

```rust
struct LinearLight(u16);
type LinearRGB = RGB<LinearLight>;
```


### `BGRA`, `ARGB`, `Gray`, etc.

There are other color types in `rgb::alt::*`. To enable `ARGB` and `ABGR`, use the "argb" feature:

```toml
rgb = { version = "0.8", features = ["argb"] }
```

There's also an optional `serde` feature that makes all types (de)serializable.
