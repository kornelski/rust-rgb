use crate::{Abgr, Argb, Bgr, Bgra, Grb, Luma, LumaA, Rgb, Rgba};
#[cfg(feature = "legacy")]
use crate::{Gray, GrayAlpha};

macro_rules! bytemuck {
    ($name:ident) => {
        unsafe impl<T> bytemuck::Zeroable for $name<T> where T: bytemuck::Zeroable {}
        unsafe impl<T> bytemuck::Pod for $name<T> where T: bytemuck::Pod {}
    };
}

bytemuck!(Rgb);
bytemuck!(Bgr);
bytemuck!(Grb);
#[cfg(feature = "legacy")]
bytemuck!(Gray);
bytemuck!(Luma);

bytemuck!(Rgba);
bytemuck!(Argb);
bytemuck!(Bgra);
bytemuck!(Abgr);
#[cfg(feature = "legacy")]
bytemuck!(GrayAlpha);
bytemuck!(LumaA);
