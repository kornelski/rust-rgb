#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![no_std]

mod formats;
mod from;
mod pixel;

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
#[cfg(feature = "legacy")]
pub use formats::gray::Gray;
pub use formats::gray_alpha::GrayAlpha;
pub use formats::grb::Grb;
pub use formats::luma::Luma;
pub use formats::luma_a::LumaA;
pub use formats::rgb::Rgb;
pub use formats::rgba::Rgba;

pub use pixel::{
    arraylike::ArrayLike,
    gain_lose_alpha::{GainAlpha, LoseAlpha},
    has_alpha::HasAlpha,
    het_pixel::{HetPixel, TryFromColorsAlphaError},
    hom_pixel::{HomPixel, TryFromComponentsError},
    pixel_component::PixelComponent,
};

/// The legacy alt module.
#[cfg(feature = "legacy")]
pub mod alt {
    pub use crate::Abgr as ABGR;
    pub use crate::Argb as ARGB;
    pub use crate::Bgr as BGR;
    pub use crate::Bgra as BGRA;
    pub use crate::Gray;
    pub use crate::GrayAlpha;
    pub use crate::Grb as GRB;
}
#[cfg(feature = "legacy")]
pub use crate::Rgb as RGB;
#[cfg(feature = "legacy")]
pub use crate::Rgba as RGBA;
