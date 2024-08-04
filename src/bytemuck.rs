use crate::{Abgr, Argb, Bgr, Bgra, Gray_v09, GrayA, Grb, Rgb, Rgba, Rgbw};

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
bytemuck!(Gray_v09);
bytemuck!(Rgbw);
bytemuck!(Rgba);
bytemuck!(Argb);
bytemuck!(Bgra);
bytemuck!(Abgr);
bytemuck!(GrayA);

use crate::formats::gray_alpha::GrayAlpha_v08;
bytemuck!(GrayAlpha_v08);

use crate::formats::gray::Gray_v08;
bytemuck!(Gray_v08);

#[cfg(feature = "as-bytes")]
impl<T: ::bytemuck::Pod> crate::ComponentBytes<T> for [Gray_v08<T>] {
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

#[cfg(feature = "as-bytes")]
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
