pub mod abgr;
pub mod argb;
pub mod bgr;
pub mod bgra;
pub mod gray;
pub mod gray_a;
pub mod grb;
pub mod rgb;
pub mod rgba;

use crate::*;
use core::array::TryFromSliceError;

macro_rules! trait_impls {
    ($name:ident, $length:literal, [$($bit:tt),*]) => {
        impl<R, S> From<$name<R>> for [S; $length] where R: Into<S> {
            fn from(value: $name<R>) -> Self {
                [$(value.$bit.into()),*]
            }
        }
        impl<R, S> From<[R; $length]> for $name<S> where R: Into<S> {
            fn from(value: [R; $length]) -> Self {
                let mut iter = value.into_iter();
                Self{$($bit: iter.next().unwrap().into()),*}
            }
        }

        impl<T> TryFrom<&[T]> for $name<T>
        where
            T: PixelComponent,
        {
            type Error = TryFromSliceError;

            fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
                let array: [T; $length] = slice.try_into()?;
                Ok(Self::from(array))
            }
        }

        impl<T> IntoIterator for $name<T> {
            type Item = T;
            type IntoIter = core::array::IntoIter<T, $length>;

            fn into_iter(self) -> Self::IntoIter {
                [$(self.$bit.into()),*].into_iter()
            }
        }
    };
}

trait_impls!(Rgb, 3, [r, g, b]);
trait_impls!(Bgr, 3, [b, g, r]);
trait_impls!(Grb, 3, [g, r, b]);
trait_impls!(Rgba, 4, [r, g, b, a]);
trait_impls!(Argb, 4, [a, r, g, b]);
trait_impls!(Bgra, 4, [b, g, r, a]);
trait_impls!(Abgr, 4, [a, b, g, r]);
trait_impls!(Gray, 1, [0]);
trait_impls!(GrayA, 2, [0, 1]);
