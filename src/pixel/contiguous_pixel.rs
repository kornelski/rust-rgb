use crate::*;

/// An trait for converting to pixel from a slice.
pub trait ContiguousPixel: Pixel {
    /// Returns a reference to the pixel type from a slice with the same number of components.
    fn from_component_slice_ref(slice: &[Self::Component]) -> &Self;
    /// Returns a mutable reference to the pixel type from a slice with the same number of components.
    fn from_component_slice_mut(slice: &mut [Self::Component]) -> &mut Self;
}

macro_rules! implement_contiguous_pixel {
    ($pixel:ident) => {
        impl<T> ContiguousPixel for $pixel<T>
        where
            T: PixelComponent,
        {
            fn from_component_slice_ref(slice: &[Self::Component]) -> &Self {
                assert_eq!(
                    slice.len(),
                    usize::from(<$pixel<T> as Pixel>::COMPONENT_COUNT)
                );
                unsafe { &*(slice.as_ptr() as *const $pixel<T>) }
            }
            fn from_component_slice_mut(slice: &mut [Self::Component]) -> &mut Self {
                assert_eq!(
                    slice.len(),
                    usize::from(<$pixel<T> as Pixel>::COMPONENT_COUNT)
                );
                unsafe { &mut *(slice.as_mut_ptr() as *mut $pixel<T>) }
            }
        }
    };
}
//This is safe since we use #[repr(C)] for the pixel struct definitions to ensure contiguous
//layout.
implement_contiguous_pixel!(Rgb);
implement_contiguous_pixel!(Bgr);
implement_contiguous_pixel!(Rgba);
implement_contiguous_pixel!(Argb);
implement_contiguous_pixel!(Bgra);
implement_contiguous_pixel!(Abgr);
implement_contiguous_pixel!(Gray);
implement_contiguous_pixel!(GrayAlpha);
