//! Basic struct for `RGB` and `RGBA` pixels. Packed, with red first, alpha last.
//!
//! This crate is intended to be the lowest common denominator for sharing `RGB`/`RGBA` bitmaps between other crates.
//!
//! The crate includes convenience functions for converting between the struct and bytes,
//! and overloaded operators that work on all channels at once.
//!
//! This crate intentionally doesn't implement color management and other colorspaces.

mod internal {
    pub mod rgb;
    pub mod rgba;
    pub mod pixel;
    pub mod ops;
    pub mod convert;
}

pub use internal::rgb::*;
pub use internal::rgba::*;
pub use internal::pixel::*;
pub use internal::ops::*;
pub use internal::convert::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
// This is it. The component type can be `u8` (aliased as `RGB8`), `u16` (aliased as `RGB16`), or any other type (but simple copyable types are recommended.)
pub struct RGB<ComponentType> {
    /// Red
    pub r:ComponentType,
    /// Green
    pub g:ComponentType,
    /// Blue
    pub b:ComponentType,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
// This is it. The component type can be `u8` (aliased as `RGBA8`), `u16` (aliased as `RGBA16`), or any other type (but simple copyable types are recommended.)
pub struct RGBA<ComponentType> {
    /// Red
    pub r:ComponentType,
    /// Green
    pub g:ComponentType,
    /// Blue
    pub b:ComponentType,
    /// Alpha
    pub a:ComponentType,
}

/// 8-bit RGB. The colorspace is techincally undefined, but generally sRGB is assumed.
pub type RGB8 = RGB<u8>;

/// 16-bit RGB in machine's native endian. Be careful to perform byte-swapping when reading from files.
pub type RGB16 = RGB<u16>;

/// Alpha is last. The crate doesn't impose which value represents transparency, but usually it's 0 = transparent, 255 = opaque.
pub type RGBA8 = RGBA<u8>;

/// 16-bit RGB in machine's native endian. Alpha is last.
pub type RGBA16 = RGBA<u16>;

#[test]
fn rgb_works() {
    let rgb = RGB{r:0u8,g:128,b:255}.clone();
    assert_eq!(rgb.b, 255);

    assert_eq!(rgb, rgb.iter().map(|ch| ch).collect());

    assert_eq!(0, rgb.as_bytes()[0]);
    assert_eq!(128, rgb.as_bytes()[1]);
    assert_eq!(255, rgb.as_bytes()[2]);

    let rgb = RGB16{r:0u16,g:0x7F7F,b:65535};
    assert_eq!(rgb.b, 65535);
    assert_eq!(rgb.as_slice()[1], 0x7F7F);

    assert_eq!(0, rgb.as_bytes()[0]);
    assert_eq!(0, rgb.as_bytes()[1]);
    assert_eq!(0x7F, rgb.as_bytes()[2]);
    assert_eq!(0x7F, rgb.as_bytes()[3]);
    assert_eq!(0xFF, rgb.as_bytes()[4]);
    assert_eq!(0xFF, rgb.as_bytes()[5]);

    assert_eq!("rgb(1,2,3)", format!("{}", RGB::new(1,2,3)));
}

#[test]
fn sub_floats() {
    assert_eq!(RGBA{r:2.5_f64, g:-1.5, b:0., a:5.}, RGBA{r:3.5_f64, g:-0.5, b:-2., a:0.} - RGBA{r:1.0_f64, g:1., b:-2., a:-5.});
}

#[test]
fn into() {
    let a:RGB8 = RGB{r:0,g:1,b:2};
    let b:RGB<i16> = a.into();
    let c:RGB<f32> = b.into();
    let d:RGB<f32> = a.into();
    assert_eq!(c, d);
}

#[test]
fn rgba_works() {
    let rgba = RGBA{r:0u8,g:128,b:255,a:33}.clone();
    assert_eq!(rgba.b, 255);
    assert_eq!(rgba.a, 33);

    assert_eq!(rgba, rgba.iter().map(|ch| ch).collect());

    assert_eq!("rgba(1,2,3,4)", format!("{}", RGBA::new(1,2,3,4)));

    assert_eq!(rgba - rgba, RGBA::new(0,0,0,0));
}
