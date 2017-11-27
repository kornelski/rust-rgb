use std::convert::*;
use super::pixel::*;
use std::slice;
use std::mem;
use RGB;
use RGBA;
use alt::BGR;
use alt::BGRA;

/// Cast a slice of component values (bytes) as a slice of RGB/RGBA pixels
///
/// If there's any incomplete pixel at the end of the slice it is ignored.
pub trait FromSlice<T: Copy> {
    /// Reinterpert slice as RGB pixels
    fn as_rgb(&self) -> &[RGB<T>];
    /// Reinterpert slice as RGBA pixels
    fn as_rgba(&self) -> &[RGBA<T>];
    /// Reinterpert mutable slice as RGB pixels
    fn as_rgb_mut(&mut self) -> &mut [RGB<T>];
    /// Reinterpert mutable slice as RGBA pixels
    fn as_rgba_mut(&mut self) -> &mut [RGBA<T>];

    /// Reinterpert slice as reverse-order BGR pixels
    fn as_bgr(&self) -> &[BGR<T>];
    /// Reinterpert slice as reverse-order BGRA pixels
    fn as_bgra(&self) -> &[BGRA<T>];
    /// Reinterpert ntable slice as reverse-order BGR pixels
    fn as_bgr_mut(&mut self) -> &mut [BGR<T>];
    /// Reinterpert mutable slice as reverse-order BGRA pixels
    fn as_bgra_mut(&mut self) -> &mut [BGRA<T>];
}

impl<T: Copy> FromSlice<T> for [T] {
    fn as_rgb(&self) -> &[RGB<T>] {
        debug_assert_eq!(3, mem::size_of::<RGB<T>>() / mem::size_of::<T>());
        unsafe {
            slice::from_raw_parts(self.as_ptr() as *const _, self.len() / 3)
        }
    }
    fn as_rgba(&self) -> &[RGBA<T>] {
        debug_assert_eq!(4, mem::size_of::<RGBA<T>>() / mem::size_of::<T>());
        unsafe {
            slice::from_raw_parts(self.as_ptr() as *const _, self.len() / 4)
        }
    }
    fn as_rgb_mut(&mut self) -> &mut [RGB<T>] {
        debug_assert_eq!(3, mem::size_of::<RGB<T>>() / mem::size_of::<T>());
        unsafe {
            slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len() / 3)
        }
    }
    fn as_rgba_mut(&mut self) -> &mut [RGBA<T>] {
        debug_assert_eq!(4, mem::size_of::<RGBA<T>>() / mem::size_of::<T>());
        unsafe {
            slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len() / 4)
        }
    }

    fn as_bgr(&self) -> &[BGR<T>] {
        debug_assert_eq!(3, mem::size_of::<BGR<T>>() / mem::size_of::<T>());
        unsafe {
            slice::from_raw_parts(self.as_ptr() as *const _, self.len() / 3)
        }
    }
    fn as_bgra(&self) -> &[BGRA<T>] {
        debug_assert_eq!(4, mem::size_of::<BGRA<T>>() / mem::size_of::<T>());
        unsafe {
            slice::from_raw_parts(self.as_ptr() as *const _, self.len() / 4)
        }
    }
    fn as_bgr_mut(&mut self) -> &mut [BGR<T>] {
        debug_assert_eq!(3, mem::size_of::<BGR<T>>() / mem::size_of::<T>());
        unsafe {
            slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len() / 3)
        }
    }
    fn as_bgra_mut(&mut self) -> &mut [BGRA<T>] {
        debug_assert_eq!(4, mem::size_of::<BGRA<T>>() / mem::size_of::<T>());
        unsafe {
            slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len() / 4)
        }
    }
}

macro_rules! rgb_impl_from {
    ($typename:ident, $from:ty, $to:ty) => {
        impl From<$typename<$from>> for $typename<$to> {

            #[inline(always)]
            fn from(other: $typename<$from>) -> Self {
                other.map(|c|c.into())
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

impl<T> From<RGB<T>> for BGR<T> {
    fn from(other: RGB<T>) -> Self {
        Self {
            r: other.r,
            g: other.g,
            b: other.b,
        }
    }
}

impl<T> From<RGBA<T>> for BGRA<T> {
    fn from(other: RGBA<T>) -> Self {
        Self {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

impl<T> From<BGR<T>> for RGB<T> {
    fn from(other: BGR<T>) -> Self {
        Self {
            r: other.r,
            g: other.g,
            b: other.b,
        }
    }
}

impl<T> From<BGRA<T>> for RGBA<T> {
    fn from(other: BGRA<T>) -> Self {
        Self {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

#[test]
fn converts() {
    assert_eq!(RGBA::new(1u8,2,3,255), RGB::new(1u8,2,3).into());
    assert_eq!(BGRA{r:1u8,g:2u8,b:3u8,a:255u8}, BGR{r:1u8,g:2u8,b:3u8}.into());
    assert_eq!(RGBA {r:1u8,g:2,b:3,a:4u8}, BGRA{r:1u8,g:2u8,b:3u8,a:4u8}.into());
    assert_eq!(BGR {r:1u8,g:2,b:3u8}, RGB {r:1u8,g:2,b:3u8}.into());
    assert_eq!(RGB {r:1u16,g:0x5678,b:0xABCDu16}, BGR {r:1u16,g:0x5678,b:0xABCDu16}.into());
    assert_eq!(BGR {r:0x1234567u32,g:2,b:3u32}, RGB {r:0x1234567u32,g:2,b:3u32}.into());
}

