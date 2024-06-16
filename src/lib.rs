#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![no_std]

mod formats;
mod from;
mod pixel;

pub use formats::abgr::Abgr;
pub use formats::argb::Argb;
pub use formats::bgr::Bgr;
pub use formats::bgra::Bgra;
pub use formats::gray::Gray;
pub use formats::gray_a::GrayA;
pub use formats::grb::Grb;
pub use formats::rgb::Rgb;
pub use formats::rgba::Rgba;

pub use pixel::{
    arraylike::ArrayLike,
    contiguous_pixel::ContiguousPixel,
    gain_alpha::{GainAlpha, LoseAlpha},
    has_alpha::HasAlpha,
    heterogeneous_pixel::HeterogeneousPixel,
    homogeneous_pixel::HomogeneousPixel,
    pixel_component::PixelComponent,
};
