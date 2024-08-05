use crate::alt::*;
use crate::formats::gray::Gray_v08;
use crate::pixel_traits::pixel::Pixel;
use crate::*;
use core::{mem, slice};

/// Casts a slice of bytes into a slice of pixels, e.g. `[u8]` to `[RGB8]`.
///
/// See also `FromSlice`
pub trait AsPixels<PixelType> {
    /// Reinterpret the slice as a read-only/shared slice of pixels.
    /// Multiple consecutive elements in the slice are interpreted as a single pixel
    /// (depending on format, e.g. 3 for RGB, 4 for RGBA).
    ///
    /// Leftover elements are ignored if the slice isn't evenly divisible into pixels.
    ///
    /// Use this method only when the type is known from context.
    /// See also `FromSlice`.
    fn as_pixels(&self) -> &[PixelType];
    /// Reinterpret the slice as a mutable/exclusive slice of pixels.
    /// Multiple consecutive elements in the slice are interpreted as a single pixel
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
    };
}

as_pixels_impl! {Rgb, 3}
as_pixels_impl! {Rgba, 4}
as_pixels_impl! {Bgr, 3}
as_pixels_impl! {Bgra, 4}
as_pixels_impl! {Grb, 3}
as_pixels_impl! {Gray_v08, 1}
as_pixels_impl! {GrayAlpha, 2}
as_pixels_impl! {Argb, 4}
as_pixels_impl! {Abgr, 4}

/// Cast a slice of component values (bytes) as a slice of RGB/RGBA pixels
///
/// If there's any incomplete pixel at the end of the slice it is ignored.
pub trait FromSlice<T: Copy> {
    /// Reinterpert slice as RGB pixels
    fn as_rgb(&self) -> &[Rgb<T>];
    /// Reinterpert slice as RGBA pixels
    fn as_rgba(&self) -> &[Rgba<T>];
    /// Reinterpert slice as alpha-first ARGB pixels
    fn as_argb(&self) -> &[ARGB<T>];
    /// Reinterpert mutable slice as RGB pixels
    fn as_rgb_mut(&mut self) -> &mut [Rgb<T>];
    /// Reinterpert mutable slice as RGBA pixels
    fn as_rgba_mut(&mut self) -> &mut [Rgba<T>];
    /// Reinterpert mutable slice as alpha-first ARGB pixels
    fn as_argb_mut(&mut self) -> &mut [ARGB<T>];

    /// Reinterpert mutable slice as grayscale pixels
    #[deprecated(note = "use bytemuck::cast_slice()")]
    fn as_gray(&self) -> &[Gray_v08<T>];
    /// Reinterpert mutable slice as grayscale pixels with alpha
    #[deprecated(note = "use bytemuck::cast_slice()")]
    fn as_gray_alpha(&self) -> &[GrayAlpha<T>];
    /// Reinterpert mutable slice as grayscale pixels
    #[deprecated(note = "use bytemuck::cast_slice()")]
    fn as_gray_mut(&mut self) -> &mut [Gray_v08<T>];
    /// Reinterpert mutable slice as grayscale pixels with alpha
    #[deprecated(note = "use bytemuck::cast_slice()")]
    fn as_gray_alpha_mut(&mut self) -> &mut [GrayAlpha<T>];

    /// Reinterpert slice as reverse-order BGR pixels
    fn as_bgr(&self) -> &[Bgr<T>];
    /// Reinterpert slice as reverse-order BGRA pixels
    fn as_bgra(&self) -> &[Bgra<T>];
    /// Reinterpert slice as reverse-order ABGR pixels
    fn as_abgr(&self) -> &[Abgr<T>];
    /// Reinterpert ntable slice as reverse-order BGR pixels
    fn as_bgr_mut(&mut self) -> &mut [Bgr<T>];
    /// Reinterpert mutable slice as reverse-order alpha-last BGRA pixels
    fn as_bgra_mut(&mut self) -> &mut [Bgra<T>];
    /// Reinterpert mutable slice as reverse-order alpha-first ABGR pixels
    fn as_abgr_mut(&mut self) -> &mut [Abgr<T>];
}

impl<T: Copy> FromSlice<T> for [T] {
    #[inline]
    fn as_rgb(&self) -> &[Rgb<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_rgba(&self) -> &[Rgba<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_argb(&self) -> &[ARGB<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_rgb_mut(&mut self) -> &mut [Rgb<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_rgba_mut(&mut self) -> &mut [Rgba<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_argb_mut(&mut self) -> &mut [ARGB<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_gray(&self) -> &[Gray_v08<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_gray_alpha(&self) -> &[GrayAlpha<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_gray_mut(&mut self) -> &mut [Gray_v08<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_gray_alpha_mut(&mut self) -> &mut [GrayAlpha<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_bgr(&self) -> &[Bgr<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_abgr(&self) -> &[Abgr<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_bgra(&self) -> &[Bgra<T>] {
        unsafe { from_items_to_struct(self) }
    }

    #[inline]
    fn as_bgr_mut(&mut self) -> &mut [Bgr<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_bgra_mut(&mut self) -> &mut [Bgra<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }

    #[inline]
    fn as_abgr_mut(&mut self) -> &mut [Abgr<T>] {
        unsafe { from_items_to_struct_mut(self) }
    }
}

#[inline(always)]
unsafe fn from_items_to_struct<F, T>(from: &[F]) -> &[T] {
    debug_assert_eq!(0, mem::size_of::<T>() % mem::size_of::<F>());
    let len = from.len() / (mem::size_of::<T>() / mem::size_of::<F>());
    slice::from_raw_parts(from.as_ptr().cast::<T>(), len)
}

#[inline(always)]
unsafe fn from_items_to_struct_mut<F, T>(from: &mut [F]) -> &mut [T] {
    debug_assert_eq!(0, mem::size_of::<T>() % mem::size_of::<F>());
    let len = from.len() / (mem::size_of::<T>() / mem::size_of::<F>());
    slice::from_raw_parts_mut(from.as_mut_ptr().cast::<T>(), len)
}

macro_rules! rgb_impl_from {
    ($typename:ident, $from:ty, $to:ty) => {
        impl From<$typename<$from>> for $typename<$to> {
            #[inline(always)]
            fn from(other: $typename<$from>) -> Self {
                other.map(core::convert::Into::into)
            }
        }
    };
}

rgb_impl_from! {Rgb, u8,i16}
rgb_impl_from! {Rgb, u8,i32}
rgb_impl_from! {Rgb, u8,u16}
rgb_impl_from! {Rgb, u8,u32}
rgb_impl_from! {Rgb, u16,i32}
rgb_impl_from! {Rgb, u16,u32}
rgb_impl_from! {Rgb, u16,u64}

rgb_impl_from! {Rgb, u8,f32}
rgb_impl_from! {Rgb, u8,f64}
rgb_impl_from! {Rgb, u16,f32}
rgb_impl_from! {Rgb, u16,f64}

rgb_impl_from! {Rgb, i16,f32}
rgb_impl_from! {Rgb, i16,f64}

rgb_impl_from! {Rgb, i32,f64}
rgb_impl_from! {Rgb, f32,f64}

rgb_impl_from! {Rgba, u16,i32}
rgb_impl_from! {Rgba, u16,u32}
rgb_impl_from! {Rgba, u16,u64}

rgb_impl_from! {Rgba, u8,i16}
rgb_impl_from! {Rgba, u8,u16}
rgb_impl_from! {Rgba, u8,u32}
rgb_impl_from! {Rgba, u8,f32}
rgb_impl_from! {Rgba, u8,f64}
rgb_impl_from! {Rgba, u16,f32}
rgb_impl_from! {Rgba, u16,f64}

rgb_impl_from! {Rgba, i16,f32}
rgb_impl_from! {Rgba, i16,f64}

rgb_impl_from! {Rgba, i32,f64}
rgb_impl_from! {Rgba, f32,f64}

impl<T: Clone> From<Gray_v08<T>> for Rgb<T> {
    #[inline(always)]
    fn from(other: Gray_v08<T>) -> Self {
        Self {
            r: other.0.clone(),
            g: other.0.clone(),
            b: other.0,
        }
    }
}

impl<T: Clone> From<Gray_v08<T>> for Rgba<T, u8> {
    #[inline(always)]
    fn from(other: Gray_v08<T>) -> Self {
        Self {
            r: other.0.clone(),
            g: other.0.clone(),
            b: other.0,
            a: 255,
        }
    }
}

impl<T: Clone, A> From<GrayAlpha<T, A>> for Rgba<T, A> {
    #[inline(always)]
    fn from(other: GrayAlpha<T, A>) -> Self {
        Self {
            r: other.0.clone(),
            g: other.0.clone(),
            b: other.0,
            a: other.1,
        }
    }
}

impl<T> AsRef<T> for Gray_v08<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsRef<T> for GrayAlpha<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Gray_v08<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> AsMut<T> for GrayAlpha<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

#[test]
fn argb_converts() {
    let argb = Argb { a: 0xffu8, r: 0xfa, g: 0xfb, b: 0xfc };
    let rgba = Rgba { a: 0xffu8, r: 0xfa, g: 0xfb, b: 0xfc };

    assert_eq!(Rgba::from(argb), rgba);
    assert_eq!(Argb::from(rgba), argb);
    assert_eq!(rgba.rgb(), argb.rgb());

    let bgra = Bgra { a: 0xffu8, r: 0x1f, g: 0x2f, b: 0x3f };
    let abgr = Abgr { a: 0xffu8, r: 0x1f, g: 0x2f, b: 0x3f };

    assert_eq!(Bgra::from(abgr), bgra);
    assert_eq!(Abgr::from(bgra), abgr);
}

#[test]
fn converts() {
    #![allow(deprecated)]
    use super::pixel::ComponentSlice;

    assert_eq!([1,2].as_gray(), [Gray_v08::new(1), Gray_v08::new(2)]);
    assert_eq!([3].as_gray_mut(), [Gray_v08::new(3)]);
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

