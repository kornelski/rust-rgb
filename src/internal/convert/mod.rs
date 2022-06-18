use super::pixel::*;
use crate::alt::*;
use crate::RGB;
use crate::RGBA;
use core::convert::*;
use core::mem;
use core::slice;

mod array;
mod tuple;

/// Casts a slice of bytes into a slice of pixels, e.g. `[u8]` to `[RGB8]`.
///
/// See also `FromSlice`
pub trait AsPixels<PixelType> {
    /// Reinterpret the slice as a read-only/shared slice of pixels.
    /// Multiple consecutive elements in the slice are intepreted as a single pixel
    /// (depending on format, e.g. 3 for RGB, 4 for RGBA).
    ///
    /// Leftover elements are ignored if the slice isn't evenly divisible into pixels.
    ///
    /// Use this method only when the type is known from context.
    /// See also `FromSlice`.
    fn as_pixels(&self) -> &[PixelType];
    /// Reinterpret the slice as a mutable/exclusive slice of pixels.
    /// Multiple consecutive elements in the slice are intepreted as a single pixel
    /// (depending on format, e.g. 3 for RGB, 4 for RGBA).
    ///
    /// Leftover elements are ignored if the slice isn't evenly divisible into pixels.
    ///
    /// Use this method only when the type is known from context.
    /// See also `FromSlice`.
    fn as_pixels_mut(&mut self) -> &mut [PixelType];
}

macro_rules! as_pixels_impl {
    ($typ:ident, $elems:expr) => {
        impl<T> AsPixels<$typ<T>> for [T] {
            fn as_pixels(&self) -> &[$typ<T>] {
                unsafe {
                    slice::from_raw_parts(self.as_ptr() as *const _, self.len() / $elems)
                }
            }
            fn as_pixels_mut(&mut self) -> &mut [$typ<T>] {
                unsafe {
                    slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() / $elems)
                }
            }
        }
    }
}

as_pixels_impl!{RGB, 3}
as_pixels_impl!{RGBA, 4}
as_pixels_impl!{BGR, 3}
as_pixels_impl!{BGRA, 4}
as_pixels_impl!{Gray, 1}
as_pixels_impl!{GrayAlpha, 2}
#[cfg(feature = "argb")]
as_pixels_impl!{ARGB, 2}
#[cfg(feature = "argb")]
as_pixels_impl!{ABGR, 2}

/// Cast a slice of component values (bytes) as a slice of RGB/RGBA pixels
///
/// If there's any incomplete pixel at the end of the slice it is ignored.
pub trait FromSlice<T: Copy> {
    /// Reinterpert slice as RGB pixels
    fn as_rgb(&self) -> &[RGB<T>];
    /// Reinterpert slice as RGBA pixels
    fn as_rgba(&self) -> &[RGBA<T>];
    /// Reinterpert slice as alpha-first ARGB pixels
    #[cfg(feature = "argb")]
    fn as_argb(&self) -> &[ARGB<T>];
    /// Reinterpert mutable slice as RGB pixels
    fn as_rgb_mut(&mut self) -> &mut [RGB<T>];
    /// Reinterpert mutable slice as RGBA pixels
    fn as_rgba_mut(&mut self) -> &mut [RGBA<T>];
    /// Reinterpert mutable slice as alpha-first ARGB pixels
    #[cfg(feature = "argb")]
    fn as_argb_mut(&mut self) -> &mut [ARGB<T>];

    /// Reinterpert mutable slice as grayscale pixels
    fn as_gray(&self) -> &[Gray<T>];
    /// Reinterpert mutable slice as grayscale pixels with alpha
    fn as_gray_alpha(&self) -> &[GrayAlpha<T>];
    /// Reinterpert mutable slice as grayscale pixels
    fn as_gray_mut(&mut self) -> &mut [Gray<T>];
    /// Reinterpert mutable slice as grayscale pixels with alpha
    fn as_gray_alpha_mut(&mut self) -> &mut [GrayAlpha<T>];

    /// Reinterpert slice as reverse-order BGR pixels
    fn as_bgr(&self) -> &[BGR<T>];
    /// Reinterpert slice as reverse-order BGRA pixels
    fn as_bgra(&self) -> &[BGRA<T>];
    /// Reinterpert slice as reverse-order ABGR pixels
    #[cfg(feature = "argb")]
    fn as_abgr(&self) -> &[ABGR<T>];
    /// Reinterpert ntable slice as reverse-order BGR pixels
    fn as_bgr_mut(&mut self) -> &mut [BGR<T>];
    /// Reinterpert mutable slice as reverse-order alpha-last BGRA pixels
    fn as_bgra_mut(&mut self) -> &mut [BGRA<T>];
    /// Reinterpert mutable slice as reverse-order alpha-first ABGR pixels
    #[cfg(feature = "argb")]
    fn as_abgr_mut(&mut self) -> &mut [ABGR<T>];
}

impl<T: Copy> FromSlice<T> for [T] {
    #[inline]
    fn as_rgb(&self) -> &[RGB<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_rgba(&self) -> &[RGBA<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    #[cfg(feature = "argb")]
    fn as_argb(&self) -> &[ARGB<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_rgb_mut(&mut self) -> &mut [RGB<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_rgba_mut(&mut self) -> &mut [RGBA<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    #[cfg(feature = "argb")]
    fn as_argb_mut(&mut self) -> &mut [ARGB<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_gray(&self) -> &[Gray<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_gray_alpha(&self) -> &[GrayAlpha<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_gray_mut(&mut self) -> &mut [Gray<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_gray_alpha_mut(&mut self) -> &mut [GrayAlpha<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }


    #[inline]
    fn as_bgr(&self) -> &[BGR<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    #[cfg(feature = "argb")]
    fn as_abgr(&self) -> &[ABGR<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_bgra(&self) -> &[BGRA<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_bgr_mut(&mut self) -> &mut [BGR<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_bgra_mut(&mut self) -> &mut [BGRA<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    #[cfg(feature = "argb")]
    fn as_abgr_mut(&mut self) -> &mut [ABGR<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }
}

#[inline(always)]
unsafe fn from_items_to_struct<F, T>(from: &[F]) -> &[T] {
    debug_assert_eq!(0, mem::size_of::<T>() % mem::size_of::<F>());
    let len = from.len() / (mem::size_of::<T>() / mem::size_of::<F>());
    slice::from_raw_parts(from.as_ptr() as *const T, len)
}

#[inline(always)]
unsafe fn from_items_to_struct_mut<F, T>(from: &mut [F]) -> &mut [T] {
    debug_assert_eq!(0, mem::size_of::<T>() % mem::size_of::<F>());
    let len = from.len() / (mem::size_of::<T>() / mem::size_of::<F>());
    slice::from_raw_parts_mut(from.as_mut_ptr() as *mut T, len)
}

macro_rules! rgb_impl_from {
    ($typename:ident, $from:ty, $to:ty) => {
        impl From<$typename<$from>> for $typename<$to> {

            #[inline(always)]
            fn from(other: $typename<$from>) -> Self {
                other.map(core::convert::Into::into)
            }
        }
    }
}

rgb_impl_from!{RGB, u8,i16}
rgb_impl_from!{RGB, u16,i32}

rgb_impl_from!{RGB, u8,f32}
rgb_impl_from!{RGB, u8,f64}
rgb_impl_from!{RGB, u16,f32}
rgb_impl_from!{RGB, u16,f64}

rgb_impl_from!{RGB, i16,f32}
rgb_impl_from!{RGB, i16,f64}

rgb_impl_from!{RGB, i32,f64}
rgb_impl_from!{RGB, f32,f64}


rgb_impl_from!{RGBA, u16,i32}

rgb_impl_from!{RGBA, u8,f32}
rgb_impl_from!{RGBA, u8,f64}
rgb_impl_from!{RGBA, u16,f32}
rgb_impl_from!{RGBA, u16,f64}

rgb_impl_from!{RGBA, i16,f32}
rgb_impl_from!{RGBA, i16,f64}

rgb_impl_from!{RGBA, i32,f64}
rgb_impl_from!{RGBA, f32,f64}

macro_rules! reorder_impl_from {
    (@rgb $t1:ident, $t2:ident) => {
        reorder_impl_from!(@once $t1, $t2, r, g, b);
        reorder_impl_from!(@once $t2, $t1, r, g, b);
    };
    (@rgba $t1:ident, $t2:ident) => {
        reorder_impl_from!(@once $t1, $t2, r, g, b, a);
        reorder_impl_from!(@once $t2, $t1, r, g, b, a);
    };
    (@once $t1:ident, $t2:ident, $($component:ident),+) => {
        impl<T> From<$t1<T>> for $t2<T> where T: ::core::clone::Clone {
            fn from(other: $t1<T>) -> Self {
                let $t1 { $($component),+ } = other;
                Self {
                    $($component),+
                }
            }
        }
    }
}

#[cfg(feature = "argb")]
reorder_impl_from!(@rgba RGBA, ARGB);
#[cfg(feature = "argb")]
reorder_impl_from!(@rgba ABGR, ARGB);
#[cfg(feature = "argb")]
reorder_impl_from!(@rgba BGRA, ARGB);
#[cfg(feature = "argb")]
reorder_impl_from!(@rgba BGRA, ABGR);

reorder_impl_from!(@rgb RGB, BGR);
reorder_impl_from!(@rgba BGRA, RGBA);
#[cfg(feature = "argb")]
reorder_impl_from!(@rgba ABGR, RGBA);

impl<T: Clone> From<Gray<T>> for RGB<T> {
    #[inline(always)]
    fn from(other: Gray<T>) -> Self {
        Self {
            r: other.0.clone(),
            g: other.0.clone(),
            b: other.0,
        }
    }
}

impl<T: Clone> From<Gray<T>> for RGBA<T, u8> {
    #[inline(always)]
    fn from(other: Gray<T>) -> Self {
        Self {
            r: other.0.clone(),
            g: other.0.clone(),
            b: other.0,
            a: 255,
        }
    }
}

impl<T: Clone,A> From<GrayAlpha<T,A>> for RGBA<T,A> {
    #[inline(always)]
    fn from(other: GrayAlpha<T,A>) -> Self {
        Self {
            r: other.0.clone(),
            g: other.0.clone(),
            b: other.0,
            a: other.1,
        }
    }
}

impl<T> AsRef<T> for Gray<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsRef<[T]> for RGB<T> {
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsRef<[T]> for RGBA<T> {
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsRef<T> for GrayAlpha<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        &self.0
    }
}


impl<T> AsMut<T> for Gray<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> AsMut<[T]> for RGB<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> AsMut<[T]> for RGBA<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> AsMut<T> for GrayAlpha<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

#[cfg(feature = "argb")]
#[test]
fn argb_converts() {
    let argb = ARGB {a: 0xffu8, r: 0xff, g: 0xff, b: 0xff};
    let rgba = RGBA {a: 0xffu8, r: 0xff, g: 0xff, b: 0xff};

    assert_eq!(RGBA::from(argb), rgba);
    assert_eq!(ARGB::from(rgba), argb);

    let bgra = BGRA {a: 0xffu8, r: 0xff, g: 0xff, b: 0xff};
    let abgr = ABGR {a: 0xffu8, r: 0xff, g: 0xff, b: 0xff};

    assert_eq!(BGRA::from(abgr), bgra);
    assert_eq!(ABGR::from(bgra), abgr);
}


#[test]
fn converts() {
    assert_eq!([1,2].as_gray(), [Gray::new(1), Gray::new(2)]);
    assert_eq!([3].as_gray_mut(), [Gray::new(3)]);
    assert_eq!([1,2].as_gray_alpha(), [GrayAlpha::new(1, 2)]);
    // excess bytes are ignored
    assert_eq!([1,2,3].as_gray_alpha_mut(), [GrayAlpha::new(1, 2)]);
    assert_eq!([1,2,3,4].as_gray_alpha_mut(), [GrayAlpha::new(1, 2), GrayAlpha::new(3, 4)]);

    assert_eq!(RGBA::new(1u8,2,3,255), RGB::new(1u8,2,3).into());
    assert_eq!(RGBA::new(1u16,2,3,65535), RGB::new(1u16,2,3).into());
    assert_eq!(BGRA{r:1u8,g:2u8,b:3u8,a:255u8}, BGR{r:1u8,g:2u8,b:3u8}.into());
    assert_eq!(BGRA{r:1u8,g:2u8,b:3u8,a:255u8}, RGB{r:1u8,g:2u8,b:3u8}.into());
    assert_eq!(RGBA {r:1u8,g:2,b:3,a:4u8}, BGRA{r:1u8,g:2u8,b:3u8,a:4u8}.into());
    assert_eq!(BGR {r:1u8,g:2,b:3u8}, RGB {r:1u8,g:2,b:3u8}.into());
    assert_eq!(RGB {r:1u16,g:0x5678,b:0xABCDu16}, BGR {r:1u16,g:0x5678,b:0xABCDu16}.into());
    assert_eq!(BGR {r:0x1234567u32,g:2,b:3u32}, RGB {r:0x1234567u32,g:2,b:3u32}.into());

    assert_eq!(&[1u8,2,3,4], RGBA {r:1u8,g:2,b:3,a:4u8}.as_slice());
    assert_eq!(&[1u8,2,3,4], RGBA {r:1u8,g:2,b:3,a:4u8}.as_ref());
    assert_eq!(&[1u8,2,3], RGB {r:1u8,g:2,b:3}.as_slice());
    assert_eq!(&[1u8,2,3], RGB {r:1u8,g:2,b:3}.as_ref());

    assert_eq!(&[1u8,2,3], RGB {r:1u8,g:2,b:3}.as_mut_slice());
    assert_eq!(&[1u8,2,3], RGB {r:1u8,g:2,b:3}.as_mut());
}

