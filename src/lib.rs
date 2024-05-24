#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![no_std]
// std is required to run unit tests
#[cfg(test)]
#[macro_use]
extern crate std;

mod abgr;
mod argb;
mod bgr;
mod bgra;
mod gray;
mod gray_alpha;
mod grb;
mod rgb;
mod rgba;
