#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![no_std]

mod formats;
mod from;
mod pixel;
mod with_alpha;

pub use formats::abgr::Abgr;
pub use formats::argb::Argb;
pub use formats::bgr::Bgr;
pub use formats::bgra::Bgra;
pub use formats::gray::Gray;
pub use formats::gray_a::GrayA;
pub use formats::rgb::Rgb;
pub use formats::rgba::Rgba;

pub use pixel::{
    as_slice::AsSlice, contiguous_pixel::ContiguousPixel, heterogeneous_pixel::HeterogeneousPixel,
    homogeneous_pixel::HomogeneousPixel, pixel_component::PixelComponent,
};
pub use with_alpha::{WithAlpha, WithoutAlpha};
