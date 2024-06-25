use crate::{Abgr, Argb, Bgr, Bgra, Grb, Luma, LumaA, Rgb, Rgba, Rgbw};

macro_rules! bytemuck {
    ($name:ident) => {
        unsafe impl<T> bytemuck::Zeroable for $name<T> where T: bytemuck::Zeroable {}
        unsafe impl<T> bytemuck::Pod for $name<T> where T: bytemuck::Pod {}
    };
}

bytemuck!(Rgb);
bytemuck!(Bgr);
bytemuck!(Grb);
bytemuck!(Rgbw);
bytemuck!(Luma);

bytemuck!(Rgba);
bytemuck!(Argb);
bytemuck!(Bgra);
bytemuck!(Abgr);
bytemuck!(LumaA);
