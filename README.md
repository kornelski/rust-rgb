# Pixel types for [Rust](https://www.rust-lang.org) [![crate](https://img.shields.io/crates/v/rgb.svg)](https://lib.rs/crates/rgb)

Operating on pixels as weakly-typed vectors of `u8` is error-prone and inconvenient. It's better to use vectors of pixel structs. However, Rust is so strongly typed that _your_ `Rgb` pixel struct is not compatible with _my_ `Rgb` pixel struct. So let's all use mine :P

[<img src="https://imgs.xkcd.com/comics/standards_2x.png" alt="xkcd: …there are 15 competing standards" width="500">](https://xkcd.com/927/)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rgb = "0.9"
```

## Usage

```rust
use rgb::{Rgb, Rgba, Argb, Bgr, Bgra, Abgr, Grb, Gray, GrayA};

let rgb = Rgb {r: 0, g: 0, b: 0};
let rbga = Rgba {r: 0, g: 0, b: 0, a: 0};
let argb = Argb {a: 0, r: 0, g: 0, b: 0};

let bgr = Bgr {b: 0, g: 0, r: 0};
let bgra = Bgra {b: 0, g: 0, r: 0, a: 0};
let abgr = Abgr {r: 0, g: 0, b: 0, a: 0};

let grb = Grb {g: 0, b: 0, r: 0};

let gray = Gray {v: 0};
let gray_a = GrayA {v: 0, a: 0};
```

If you have a pixel type you would like to use that is not currently
implemented, please open an issue to request your pixel type.

The pixel types with an alpha component such as `Rgba` have two
generic type parameters:

```rust
struct Rgba<T, A = T> {
    r: T,
    g: T,
    b: T,
    a: A,
}
```

This makes them more flexible for more use-cases, for example if you
needed more precision for you color components than your alpha
component you could create an `Rgba<f32, u8>`. However, in most
use-cases the alpha component type will be the same as the color
component type.

A pixel with separate types for the color and alpha
components is called a heterogeneous pixel (`HetPixel`), whereas a pixel with a
single type for both color and alpha components is called a
homogeneous pixel (`Pixel`).

## Pixel Traits

All functionality for the pixel types is implemented via traits. This
means that none of the pixel types, like `Rgb<u8>`, have any inherent
methods. This makes it easy to choose which methods you'd like to be
in scope at any given time unlike inherent methods which are always
within scope.

This crate offers the following traits:

### HetPixel

The most foundational pixel trait implemented by every pixel type.

```rust
use rgb::{Rgba, HetPixel};

let mut rgba: Rgba<u8> = Rgba::try_from_colors_alpha([0, 0, 0], 0).unwrap();

*rgba.color_array_mut()[2] = u8::MAX;
assert_eq!(rgba.color_array(), [0, 0, 255]);

*rgba.alpha_opt_mut().unwrap() = 50;
assert_eq!(rgba.alpha_opt(), Some(50));

let rgba = rgba.map_colors(u16::from);
let rgba = rgba.map_colors_same(|c| c * 2);
let rgba = rgba.map_alpha(f32::from);
let rgba = rgba.map_alpha_same(|a| a * 2.0);

assert_eq!(rgba, Rgba::<u16, f32> {r: 0, g: 0, b: 510, a: 100.0});
```

### Pixel

A stricter form of `HetPixel` where the two component types, color and
alpha, are the same.

```rust
use rgb::{Rgba, Pixel};

let mut rgba: Rgba<u8> = Rgba::try_from_components([0, 0, 0, 0]).unwrap();

*rgba.each_mut()[2] = u8::MAX;
assert_eq!(rgba.to_array(), [0, 0, 255, 0]);

let rgba = rgba.map(u16::from);
let rgba = rgba.map_same(|c| c * 2);

assert_eq!(rgba, Rgba::<u16> {r: 0, g: 0, b: 510, a: 0});
```

#### GainAlpha

A way to add alpha to a pixel type in various ways.

```rust
use rgb::{Rgb, Rgba, GainAlpha};

let expected: Rgba<u8> = Rgba {r: 0, g: 0, b: 0, a: 255};

assert_eq!(Rgb {r: 0, g: 0, b: 0}.with_default_alpha(255), expected);
assert_eq!(Rgb {r: 0, g: 0, b: 0}.with_alpha(255), expected);
assert_eq!(Rgba {r: 0, g: 0, b: 0, a: 0}.with_alpha(255), expected);
```

#### HasAlpha

A trait only implemented on pixels that have an alpha
component.

```rust
use rgb::{Rgba, HasAlpha};

let mut rgba: Rgba<u8> = Rgba {r: 0, g: 0, b: 0, a: 255};

*rgba.alpha_mut() -= 50;

assert_eq!(rgba.alpha(), 205);
```

## Bytemuck

If you have a `&[u8]` or `Vec<u8>` type and you want a `&[Rgb<u8>]` or
`Vec<Rgb<u8>>` type then you can safely achieve these type-casts via
the `bytemuck` crate (see `cast_slice()` and `cast_vec()`).

You will need to enable the `bytemuck` crate feature in order to use
functions from the `bytemuck` library on the pixel types in this
crate.

## Crate Features

- `default`: The default feature which does nothing.
- `num-traits`: Enables various
  [`num_traits`](https://docs.rs/num-traits) traits impls for the
  pixel types such as `CheckedAdd`.
- `defmt-03` = Enables the `Format` trait impls from
  [`defmt`](https://docs.rs/defmt) `v0.3` for the pixel types
- `serde` = Enables `Serializable` and `Deserializable` trait impls
  from [`serde`](https://docs.rs/serde) for the pixel types
- `bytemuck` = Enables `Pod` and `Zeroable` trait impls from
  [`bytemuck`](https://docs.rs/serde) for the pixel types

### Legacy Features

The following crate features are only exposed for compatibility with
the `v0.8` release so as to be non-breaking, however, once migrated to
`v0.9` you should no longer be using any of these features. They are
going to be removed in the next major release after `v0.9`.

```toml
legacy = ["as-bytes"]
argb = []
grb = []
checked_fns = []
as-bytes = ["bytemuck"]
```

## Color-Space Agnostic

This crate is purposefully agnostic about the color-spaces of the
pixel types. For example, `Gray<u8>` could be either linear lightness or
gamma-corrected luma, etc.

*Correct* color management is a complex problem, and this crate aims to be the lowest common denominator, so it's intentionally agnostic about it.

However, this library supports any subpixel type for `RGB<T>`, and `RGBA<RGBType, AlphaType>`, so you can use them with a newtype, e.g.:

```rust
# use rgb::Rgb;
struct LinearLight(u16);
type LinearRGB = Rgb<LinearLight>;
```

## Roadmap to 1.0

The plan is to provide easy migration to v1.0. There will be a transitional v0.9 version released that will be mostly backwards-compatible with 0.8, and forwards-compatible with 1.0.

Planned changes:

 * Types will be renamed to follow Rust's naming convention: `RGBA` → `Rgba`. The old names will continue to work as hidden aliases.
 * The `Gray` and `GrayAlpha` types will change from tuple structs with `.0` to structs with named fields `.v` (value) and `.a` (alpha). Through a `Deref` trick both field names will work, but `.0` is going to be deprecated.
 * `bytemuck::Pod` (conversions from/to raw bytes) will require color and alpha components to be the same type (i.e. it will work with `Rgba<u8>`, but not `Rgba<Newtype, DifferentType>`). Currently it's unsound if the alpha has a different size than color components.
 * Many inherent methods will be moved to a new `Pixel` trait.

## Migration from 0.8 to 0.9

1. Update to the latest version of 0.8, and fix all deprecation warnings.

   - rename `.alpha()` to `.with_alpha()`

2. Change field access on `GrayAlpha` from `.0` and `.1` to `.v` and `.a` where possible.
3. Use the `bytemuck` crate for conversions from/to bytes.
