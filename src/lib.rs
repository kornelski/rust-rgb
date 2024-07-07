#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

#[cfg(all(test, feature = "legacy"))]
#[macro_use] extern crate std;

mod formats;
mod from;
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

#[cfg(feature = "legacy")]
mod legacy;
#[cfg(feature = "legacy")]
pub use legacy::*;
#[cfg(feature = "legacy")]
pub use ::bytemuck::{Pod, Zeroable};
#[cfg(feature = "legacy")]
pub use legacy::internal::pixel::{ComponentSlice, ComponentBytes, ColorComponentMap};
#[cfg(feature = "legacy")]
pub use pixel_traits::pixel::Pixel as ComponentMap;
#[cfg(feature = "legacy")]
pub use legacy::internal::convert::{AsPixels, FromSlice};

#[cfg(feature = "num-traits")]
mod num_traits;

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
    arraylike::ArrayLike,
    gain_alpha::GainAlpha,
    has_alpha::HasAlpha,
    het_pixel::HetPixel,
    pixel::Pixel,
};

/// TryFrom errors
pub mod error {
    pub use crate::pixel_traits::het_pixel::TryFromColorsAlphaError;
    pub use crate::pixel_traits::pixel::TryFromComponentsError;

}
