use crate::{Abgr, Argb, Bgr, Bgra, Gray, GrayA, Grb, Rgb, Rgba, Rgbw};

pub use ::bytemuck::*;

macro_rules! bytemuck {
    ($name:ident) => {
        unsafe impl<T: ::bytemuck::Zeroable> ::bytemuck::Zeroable for $name<T> {}
        unsafe impl<T: ::bytemuck::Pod> ::bytemuck::Pod for $name<T> {}
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

#[cfg(feature = "legacy")]
use crate::formats::gray_alpha::GrayAlpha_v08;
#[cfg(feature = "legacy")]
bytemuck!(GrayAlpha_v08);

#[cfg(feature = "legacy")]
use crate::formats::gray::Gray_v08;
#[cfg(feature = "legacy")]
bytemuck!(Gray_v08);

#[cfg(all(feature = "legacy", feature = "as-bytes"))]
impl<T: ::bytemuck::Pod> crate::ComponentBytes<T> for [Gray<T>] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        ::bytemuck::cast_slice(self)
    }

    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        ::bytemuck::cast_slice_mut(self)
    }
}

#[cfg(all(feature = "legacy", feature = "as-bytes"))]
impl<T: ::bytemuck::Pod> crate::ComponentBytes<T> for [GrayAlpha_v08<T>] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        ::bytemuck::cast_slice(self)
    }

    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        ::bytemuck::cast_slice_mut(self)
    }
}
