#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![no_std]

mod abgr;
mod argb;
mod bgr;
mod bgra;
mod gray;
mod gray_alpha;
mod rgb;
mod rgba;

mod from;
mod pixel;
mod with_alpha;

pub use abgr::Abgr;
pub use argb::Argb;
pub use bgr::Bgr;
pub use bgra::Bgra;
pub use gray::Gray;
pub use gray_alpha::GrayA;
pub use rgb::Rgb;
pub use rgba::Rgba;

pub use pixel::{
    as_slice::AsSlice, contiguous_pixel::ContiguousPixel, heterogeneous_pixel::HeterogeneousPixel,
    homogeneous_pixel::HomogeneousPixel, pixel_component::PixelComponent,
};
pub use with_alpha::{WithAlpha, WithoutAlpha};
