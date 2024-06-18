use crate::{Abgr, Argb, Bgr, Bgra, Gray, GrayA, Grb, Rgb, Rgba};

macro_rules! bytemuck {
    ($name:ident, [$($bit:tt),*]) => {
        unsafe impl<T> bytemuck::Zeroable for $name<T> where T: bytemuck::Zeroable {}
        unsafe impl<T> bytemuck::Pod for $name<T> where T: bytemuck::Pod {}
    };
}

bytemuck!(Rgb, [r, g, b]);
bytemuck!(Bgr, [b, g, r]);
bytemuck!(Grb, [g, r, b]);
bytemuck!(Gray, [0]);
bytemuck!(Rgba, [r, g, b, a]);
bytemuck!(Argb, [a, r, g, b]);
bytemuck!(Bgra, [b, g, r, a]);
bytemuck!(Abgr, [a, b, g, r]);
bytemuck!(GrayA, [0, 1]);
