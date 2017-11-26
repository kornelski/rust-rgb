use std::convert::*;
use super::pixel::*;
use std::slice;
use std::mem;
use RGB;
use RGBA;

/// Cast a slice of component values (bytes) as a slice of RGB/RGBA pixels
///
/// If there's any incomplete pixel at the end of the slice it is ignored.
pub trait FromSlice<T: Copy> {
    fn as_rgb(&self) -> &[RGB<T>];
    fn as_rgba(&self) -> &[RGBA<T>];
    fn as_rgb_mut(&mut self) -> &mut [RGB<T>];
    fn as_rgba_mut(&mut self) -> &mut [RGBA<T>];
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
