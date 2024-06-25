use crate::{Abgr, Argb, Bgr, Bgra, Gray, GrayA, Grb, Rgb, Rgba, Rgbw};

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
bytemuck!(Rgbw);
bytemuck!(Rgba);
bytemuck!(Argb);
bytemuck!(Bgra);
bytemuck!(Abgr);
bytemuck!(GrayA);
