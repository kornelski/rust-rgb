#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![no_std]

#[cfg(feature = "bytemuck")]
mod bytemuck;
mod formats;
mod from;
#[cfg(feature = "num-traits")]
mod num_traits;
mod pixel;

pub use formats::abgr::Abgr;
pub use formats::argb::Argb;
pub use formats::bgr::Bgr;
pub use formats::bgra::Bgra;
pub use formats::gray::Gray;
pub use formats::gray_a::GrayA;
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
