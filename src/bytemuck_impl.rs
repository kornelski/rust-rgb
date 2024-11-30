use crate::{Abgr, Argb, Bgr, Bgra, GrayA, GrayAlpha44, Gray_v09, Grb, Rgb, Rgba, Rgbw};

macro_rules! bytemuck {
    ($name:ident) => {
        unsafe impl<T: ::bytemuck::Zeroable> ::bytemuck::Zeroable for $name<T> {}
        unsafe impl<T: ::bytemuck::Pod> ::bytemuck::Pod for $name<T> {}
    };
}

macro_rules! bytemuck_no_generic {
    ($name:ident) => {
        unsafe impl ::bytemuck::Zeroable for $name {}
        unsafe impl ::bytemuck::Pod for $name {}
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
bytemuck_no_generic!(GrayAlpha44);

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
impl<T: ::bytemuck::Pod> crate::ComponentBytes<T> for [Gray_v09<T>] {
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

#[cfg(feature = "as-bytes")]
impl crate::ComponentBytes<u8> for [GrayAlpha44] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        ::bytemuck::cast_slice(self)
    }

    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        ::bytemuck::cast_slice_mut(self)
    }
}

#[test]
fn test_component_bytes_capable() {
    assert_eq!(
        core::mem::size_of::<GrayAlpha44>(),
        core::mem::size_of::<u8>()
    );
}
