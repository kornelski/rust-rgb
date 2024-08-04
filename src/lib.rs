#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

#[macro_use]
extern crate std;

mod formats {
    pub mod abgr;
    pub mod argb;
    pub mod bgr;
    pub mod bgra;
    pub mod gray;
    pub mod gray_a;
    pub mod gray_alpha;
    pub mod grb;
    pub mod rgb;
    pub mod rgba;
    pub mod rgbw;
}
mod core_traits;
mod from;
mod inherent_impls;
mod tuples;
mod pixel_traits {
    pub mod arraylike;
    pub mod gain_alpha;
    pub mod has_alpha;
    pub mod het_pixel;
    pub mod pixel;
}

/// Re-export of the [`bytemuck` crate](https://lib.rs/bytemuck). [See docs](https://docs.rs/bytemuck).
///
/// Use [`::bytemuck::cast_slice()`] or [`::bytemuck::from_bytes()`] to convert
/// pixels to/from `&[u8]`.
#[cfg(feature = "bytemuck")]
#[doc(alias = "ComponentSlice")]
#[doc(alias = "as_bytes")]
#[doc(alias = "Pod")]
pub mod bytemuck;

mod legacy;
pub use ::bytemuck::{Pod, Zeroable};
pub use legacy::internal::convert::{AsPixels, FromSlice};
pub use legacy::internal::pixel::{ColorComponentMap, ComponentBytes, ComponentSlice};
pub use legacy::*;
pub use pixel_traits::pixel::Pixel as ComponentMap;

/// If the `num-traits` feature is enabled, the implemented traits are in this module
#[cfg(feature = "num-traits")]
pub mod num_traits;

pub use formats::abgr::Abgr;
pub use formats::argb::Argb;
pub use formats::bgr::Bgr;
pub use formats::bgra::Bgra;
pub use formats::gray::Gray_v09 as Gray;
pub use formats::gray_a::GrayA;
pub use formats::grb::Grb;
pub use formats::rgb::Rgb;
pub use formats::rgba::Rgba;
pub use formats::rgbw::Rgbw;

pub use pixel_traits::{
    arraylike::ArrayLike, gain_alpha::GainAlpha, has_alpha::HasAlpha, het_pixel::HetPixel,
    pixel::Pixel,
};
/// A module of re-exports of all the traits provided by this crate
/// for use with glob imports instead of importing relevant pixel
/// traits individually.
///
/// ```
/// use rgb::Rgb;
/// use rgb::prelude::*;
///
/// let pixel = Rgb::try_from_components([0, 0, 0]).unwrap();
/// ```
pub mod prelude {
    pub use crate::ArrayLike;
    pub use crate::GainAlpha;
    pub use crate::HasAlpha;
    pub use crate::HetPixel;
    pub use crate::Pixel;
}

/// TryFrom errors
pub mod error {
    pub use crate::pixel_traits::het_pixel::TryFromColorsAlphaError;
    pub use crate::pixel_traits::pixel::TryFromComponentsError;
}

/// [`Abgr<u8>`]
pub type ABGR8 = formats::abgr::Abgr<u8>;
/// [`Argb<u8>`]
pub type ARGB8 = formats::argb::Argb<u8>;
/// [`Bgr<u8>`]
pub type BGR8 = formats::bgr::Bgr<u8>;
/// [`Bgra<u8>`]
pub type BGRA8 = formats::bgra::Bgra<u8>;
/// [`Gray<u8>`]
pub type GRAY8 = formats::gray::Gray_v09<u8>;
/// [`GrayA<u8>`]
pub type GRAYA8 = formats::gray_a::GrayA<u8>;
/// [`Grb<u8>`]
pub type GRB8 = formats::grb::Grb<u8>;
/// [`Rgb<u8>`] 8-bit RGB
///
/// The colorspace is technically undefined, but generally sRGB is assumed.
pub type RGB8 = formats::rgb::Rgb<u8>;
/// [`Rgba<u8>`] 8-bit RGBA, alpha is last. 0 = transparent, 255 = opaque.
pub type RGBA8 = formats::rgba::Rgba<u8>;
/// [`Rgbw<u8>`]
pub type RGBW8 = formats::rgbw::Rgbw<u8>;

/// [`Abgr<u16>`]
pub type ABGR16 = formats::abgr::Abgr<u16>;
/// [`Argb<u16>`]
pub type ARGB16 = formats::argb::Argb<u16>;
/// [`Bgr<u16>`]
pub type BGR16 = formats::bgr::Bgr<u16>;
/// [`Bgra<u16>`]
pub type BGRA16 = formats::bgra::Bgra<u16>;
/// [`Gray<u16>`]
pub type GRAY16 = formats::gray::Gray_v09<u16>;
/// [`GrayA<u16>`]
pub type GRAYA16 = formats::gray_a::GrayA<u16>;
/// [`Grb<u16>`]
pub type GRB16 = formats::grb::Grb<u16>;
/// [`Rgb<u16>`] 16-bit RGB in machine's native endian
///
/// Be careful to perform byte-swapping when reading from files.
pub type RGB16 = formats::rgb::Rgb<u16>;
/// [`Rgba<u16>`] 16-bit RGB in machine's native endian. 0 = transparent, 65535 = opaque.
///
/// Alpha is last.
pub type RGBA16 = formats::rgba::Rgba<u16>;
/// [`Rgbw<u16>`]
pub type RGBW16 = formats::rgbw::Rgbw<u16>;

/// [`Rgba<f32>`]
pub type RGBA32F = formats::rgba::Rgba<f32>;
