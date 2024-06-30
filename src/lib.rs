#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![no_std]

mod formats;
mod from;
mod pixel_traits;

#[cfg(feature = "bytemuck")]
mod bytemuck;
#[cfg(feature = "legacy")]
mod legacy;
#[cfg(feature = "num-traits")]
mod num_traits;

pub use formats::abgr::Abgr;
pub use formats::argb::Argb;
pub use formats::bgr::Bgr;
pub use formats::bgra::Bgra;
pub use formats::grb::Grb;
pub use formats::gray::Gray;
pub use formats::gray_a::GrayA;
pub use formats::rgb::Rgb;
pub use formats::rgba::Rgba;
pub use formats::rgbw::Rgbw;

pub use pixel_traits::{
    arraylike::ArrayLike,
    gain_alpha::GainAlpha,
    has_alpha::HasAlpha,
    het_pixel::{HetPixel, TryFromColorsAlphaError},
    pixel::{Pixel, TryFromComponentsError},
    pixel_component::PixelComponent,
};

#[cfg(feature = "legacy")]
pub use crate::Rgb as RGB;
#[cfg(feature = "legacy")]
pub use crate::Rgba as RGBA;

/// Re-export from `bytemuck` crate
#[cfg(feature = "as-bytes")]
#[cfg(feature = "legacy")]
pub use ::bytemuck::Pod;
/// Re-export from `bytemuck` crate
#[cfg(feature = "as-bytes")]
#[cfg(feature = "legacy")]
pub use ::bytemuck::Zeroable;

#[cfg(feature = "legacy")]
pub use legacy::internal::convert::AsPixels;
#[cfg(feature = "legacy")]
pub use legacy::internal::convert::FromSlice;
#[cfg(feature = "legacy")]
pub use legacy::internal::pixel::ColorComponentMap;
#[cfg(feature = "legacy")]
pub use legacy::internal::pixel::ComponentBytes;
#[cfg(feature = "legacy")]
pub use legacy::internal::pixel::ComponentMap;
#[cfg(feature = "legacy")]
pub use legacy::internal::pixel::ComponentSlice;

#[cfg(feature = "legacy")]
pub use legacy::alt;

/// 8-bit RGB
///
/// The colorspace is technically undefined, but generally sRGB is assumed.
#[cfg(feature = "legacy")]
pub type RGB8 = RGB<u8>;

/// 16-bit RGB in machine's native endian
///
/// Be careful to perform byte-swapping when reading from files.
#[cfg(feature = "legacy")]
pub type RGB16 = RGB<u16>;

/// 8-bit RGBA, alpha is last. 0 = transparent, 255 = opaque.
#[cfg(feature = "legacy")]
pub type RGBA8 = RGBA<u8>;

/// 16-bit RGB in machine's native endian. 0 = transparent, 65535 = opaque.
///
/// Alpha is last.
#[cfg(feature = "legacy")]
pub type RGBA16 = RGBA<u16>;
