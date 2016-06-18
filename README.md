# `struct RGB` for [Rust](https://www.rust-lang.org)

Rust is strongly typed and *your* RGB pixel is not compatible with *my* RGB pixel.

So let's all use mine :P

```rust
extern crate rgb;

let px = RGB{r:255_u8,g:0,b:100};
assert_eq!(px.as_bytes()[0], 255);

let px = RGB8::new(255,0,255);
let inverted = px.map(|ch| 255 - ch);

println!("{}", inverted); // rgb(0,255,0)
```

[![xkcd standards](https://imgs.xkcd.com/comics/standards.png)](https://xkcd.com/927/)
