#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

#[cfg(all(test, feature = "legacy"))]
#[macro_use] extern crate std;

mod formats;
mod from;
mod pixel_traits;

#[cfg(feature = "bytemuck")]
mod bytemuck;
#[cfg(feature = "legacy")]
mod legacy;
#[cfg(feature = "legacy")]
pub use legacy::*;
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
    het_pixel::{HetPixel, TryFromColorsAlphaError},
    pixel::{Pixel, TryFromComponentsError},
    pixel_component::PixelComponent,
};
