use crate::{Abgr, Argb, Bgr, Bgra, Gray, GrayAlpha, Grb, Luma, LumaA, Rgb, Rgba};

macro_rules! bytemuck {
    ($name:ident) => {
        unsafe impl<T> bytemuck::Zeroable for $name<T> where T: bytemuck::Zeroable {}
        unsafe impl<T> bytemuck::Pod for $name<T> where T: bytemuck::Pod {}
    };
}

bytemuck!(Rgb);
bytemuck!(Bgr);
bytemuck!(Grb);
bytemuck!(Gray);
bytemuck!(Luma);

bytemuck!(Rgba);
bytemuck!(Argb);
bytemuck!(Bgra);
bytemuck!(Abgr);
bytemuck!(GrayAlpha);
bytemuck!(LumaA);
