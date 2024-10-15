# Pixel types for [Rust](https://www.rust-lang.org) [![crate](https://img.shields.io/crates/v/rgb.svg)](https://lib.rs/crates/rgb)

Operating on pixels as weakly-typed vectors of `u8` is error-prone and inconvenient. It's better to use vectors and slices of pixel structs, like `&[Rgb<u8>]`.

However, Rust is so strongly typed that _your_ `Rgb` struct is not compatible with _my_ `Rgb` struct. This crate provides common structs to share between crates.

## Features

 * It's a shared type [used in many crates](https://lib.rs/crates/rgb/rev), which allows you to seamlessly and reliably share pixel data between them.

 * Compiles quickly and has low overhead. The dependencies are only for interoperability with the broader ecosystem, and are all optional.

 * Implements standard Rust traits, and has convenience methods for operating on all channels of the pixels. Saves you from having to copy-paste the same lines for `r`, `g`, and `b`.

 * It's unopinionated about color management, which lets it prepresent most RGB-related pixels without interfering. If you need more advanced conversions and non-RGB color spaces, use the [`palette` crate](https://lib.rs/crates/palette) instead.

## Basic Usage

```rust
use rgb::{Rgb, Rgba, Argb, Bgr, Bgra, Abgr, Grb}; // and more
use rgb::prelude::*; // traits with convenience methods

let rgb_pixel = Rgb { r: 0u8, g: 100, b: 255 };
let wider_pixel = rgb_pixel.map(u16::from);

println!("{rgb_pixel}"); // prints rgb(0, 100, 255)
println!("{rgb_pixel:X}"); // prints #0064FF

assert_eq!(rgb_pixel.to_color_array(), [0, 100, 255]);
assert_eq!(rgb_pixel.with_alpha(128), Rgba::new(0, 100, 255, 128));
```

### Conversions from/to other types

We defer to the `bytemuck` crate to have safe zero-cost conversions between types. See [`bytemuck::cast_slice()`][bslice] and [`cast_vec()`][bvec].

[bslice]: https://docs.rs/bytemuck/latest/bytemuck/fn.cast_slice.html
[bvec]: https://docs.rs/bytemuck/latest/bytemuck/allocation/fn.cast_vec.html

```rust,ignore
let pixels: Vec<u8> = vec![0u8; 3 * size];
let rgb_pixels: Vec<Rgb<u8>> = rgb::bytemuck::allocation::cast_vec(pixels);

for rgb_pixel in &rgb_pixels {
}
```

If you'd like to work with 2D slices of pixels, see [the `imgvec` crate](https://lib.rs/crates/imgvec).

# Stable and testing versions

The version 0.8 is stable, and we plan to support it for a long time. You can use it, and rely on it.

```toml
[dependencies]
rgb = "0.8.50"
```

We want to release a proper v1.0.0 eventually. We plan to have it backwards-compatible with crates using v0.8, except some deprecated cruft removed/fixed. We hope the migration will be seamless for most users. Please help us test it!

```toml
# This is required due to how version unification works in Cargo
[patch.crates-io]
rgb.git = "https://github.com/kornelski/rust-rgb"

[dependencies]
rgb = "0.8.90"
```

- Are the names of the traits and their methods good?
- Are there any standard library traits you'd like implemented on the pixel types?
- Is the split between `Pixel`, `HetPixel`, `HasAlpha` sensible?
  (pixels support a different type for the alpha channel, and there's `Rgbw` without alpha).

[Please open issues in the repo with the feedback](https://github.com/kornelski/rust-rgb/issues)
or message [@kornel@mastodon.social](https://mastodon.social/@kornel).

## Usage

```rust
use rgb::{Rgb, Rgba, Argb, Bgr, Bgra, Abgr, Grb, Gray_v09 as Gray, GrayA};

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

### `HetPixel`

The most foundational pixel trait implemented by every pixel type.

```rust
use rgb::{Rgba, HetPixel};

let mut rgba: Rgba<u8> = Rgba::try_from_colors_alpha([0, 0, 0], 0).unwrap();

*rgba.each_color_mut()[2] = u8::MAX;
assert_eq!(rgba.to_color_array(), [0, 0, 255]);

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

### `GainAlpha`

A way to add alpha to a pixel type in various ways.

```rust
use rgb::{Rgb, Rgba, GainAlpha};

let expected: Rgba<u8> = Rgba {r: 0, g: 0, b: 0, a: 255};

assert_eq!(Rgb {r: 0, g: 0, b: 0}.with_default_alpha(255), expected);
assert_eq!(Rgb {r: 0, g: 0, b: 0}.with_alpha(255), expected);
assert_eq!(Rgba {r: 0, g: 0, b: 0, a: 0}.with_alpha(255), expected);
```

### `HasAlpha`

A trait only implemented on pixels that have an alpha
component.

Due to a naming conflict with several now-deprecated inherent
functions with the same name (such as `Rgb::alpha()`) the
`HasAlpha::alpha()` method requires fully qualified syntax for
disambiguation. The deprecated functions are due to be removed in a
future release which will solve this issue.

```rust
use rgb::{Rgba, HasAlpha};

let mut rgba: Rgba<u8> = Rgba {r: 0, g: 0, b: 0, a: 255};

*rgba.alpha_mut() -= 50;

assert_eq!(HasAlpha::alpha(&rgba), 205);
```

## Crate Features

- `num-traits`: Enables various
  [`num_traits`](https://docs.rs/num-traits) traits impls for the
  pixel types such as `CheckedAdd`.
- `defmt-03` = Enables the `Format` trait impls from
  [`defmt`](https://docs.rs/defmt) `v0.3` for the pixel types
- `serde` = Enables `Serializable` and `Deserializable` trait impls
  from [`serde`](https://docs.rs/serde) for the pixel types
- `bytemuck` = Enables `Pod` and `Zeroable` trait impls from
  [`bytemuck`](https://docs.rs/serde) for the pixel types

The following crate features are only kept for backwards compatibility, and will be removed in the next major version:

```toml
# These are no longer used
argb = []
grb = []
checked_fns = []
as-bytes = ["bytemuck"]
```

## Color-Space Agnostic

This crate is purposefully a basic lowest-common-denominator, and it does not dictate what color spaces the pixel types are supposed to use.
For example, `Gray<u8>` could be either linear lightness or gamma-corrected luma, however you wish to use it.
_Correct_ color management is a complex problem, and this crate doesn't want to impose any specific solutions.

If you need strongly-typed color spaces, you can use newtypes as component types for `Rgb<T>` and `Rgba<T, AlphaType>`, e.g.:

```rust
# use rgb::Rgb;
struct LinearLight(u16);
type LinearRGB = Rgb<LinearLight>;
```

## Roadmap to 1.0

The plan is to provide easy migration to v1.0. There will be a
transitional v0.9 version released that will be mostly
backwards-compatible with 0.8, and forwards-compatible with 1.0.

The changes:

- Types were renamed to follow Rust's naming convention: `RGBA` → `Rgba`.
  Type aliases with an `8` or `16` suffix (`RGBA8`) were kept as-is.
- The grayscale types have changed from being tuple structs with
  `.0`/`.1` to structs with named fields `.v` (value) and `.a` (alpha).
- `GrayAlpha` has been renamed to `GrayA`.
- `bytemuck::Pod` (conversions from/to raw bytes) require color and alpha components to be the same type
  (i.e. it works with `Rgba<u8>`, but not `Rgba<Newtype, DifferentType>`).
- Most inherent methods were moved to a new `Pixel` trait.

### Migrating away from deprecated items

Some items in this crate have become deprecated in preparation for a
future release which removes them. Here is a checklist of things you may need to do.

1. Update to the latest version of 0.8, and fix all deprecation warnings.
   - rename `.alpha()` to `.with_alpha()`
   - rename `.map_c()` to `.map_colors()`
1. Change field access on `GrayAlpha` from `.0` and `.1` to `.v` and `.a` where possible.
1. Use the `bytemuck` crate for conversions from/to bytes instead of `ComponentBytes` trait. Disable the `as-bytes` feature if possible.
1. Use the `num-traits` crate for `.checked_add()`, don't enable `checked_fns` feature.
1. Don't enable `gbr` and `argb` features. All pixel types are enabled by default.
1. `AsRef<[T]>` implementations have changed to `AsRef<[T; N]>`. In most cases `.as_ref()`/`.as_mut()` calls should coerce to a slice anyway.
1. Instead of `pixel.as_slice()` use `pixel.as_ref()`.
1. Stop using the `rgb::Gray`/`rgb::GrayAlpha` types and switch to `rgb::Gray_v09 as Gray`/`rgb::GrayA` instead respectively.
1. In generic code operating on pixels, add `Copy + 'static` bounds to the pixel types and their components.
