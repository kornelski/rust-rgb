# `struct RGB` for [Rust](https://www.rust-lang.org)

Operating on pixels as weakly-typed vectors of `u8` is error-prone and inconvenient. It's better to use vectors of pixel structs. However, Rust is so strongly typed that *your* RGB pixel struct is not compatible with *my* RGB pixel struct. So let's all use mine :P

[![xkcd standards](https://imgs.xkcd.com/comics/standards.png)](https://xkcd.com/927/)

## Usage

### `RGB` and `RGBA` structs

```rust
extern crate rgb;

let px = RGB{r:255_u8, g:0, b:100};
assert_eq!(px.as_bytes()[0], 255);

let px = RGB8::new(255, 0, 255);
let inverted = px.map(|ch| 255 - ch);

println!("{}", inverted); // rgb(0,255,0)
### Byte slices to pixel slices

For interoperability with functions operating on generic arrays of bytes there are functinos for safe casting to and from pixel slices.

```rust
let raw = vec![0u8; width*height*3];
let pixels: &[RGB8] = raw.as_rgb(); /// Safe casts without copying
let raw_again = pixels.as_bytes();
```


----

## About colorspaces

This crate is intentionally ignorant about flavors of RGB color spaces. *Correct* color management is a complex problem, and this crate aims to be the lowest common denominator.
