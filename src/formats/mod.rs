pub mod abgr;
pub mod argb;
pub mod bgr;
pub mod bgra;
pub mod gray;
pub mod gray_alpha;
pub mod grb;
pub mod luma;
pub mod luma_a;
pub mod rgb;
pub mod rgba;

use core::array::TryFromSliceError;
use core::fmt;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{Abgr, Argb, Bgr, Bgra, Grb, Luma, LumaA, PixelComponent, Rgb, Rgba};
#[cfg(feature = "legacy")]
use crate::{Gray, GrayAlpha};

macro_rules! hom_trait_impls {

    ($name:ident, 1, [$($bit:tt:$num:tt),*], $display:literal, $upperhex:literal, $lowerhex:literal) => {
        hom_trait_impls!(normal_impls, $name, 1, [$($bit:$num),*], $display, $upperhex, $lowerhex);
    };
    ($name:ident, 2, [$($bit:tt:$num:tt),*], $display:literal, $upperhex:literal, $lowerhex:literal) => {
        impl<R, S> From<$name<R>> for (S, S) where R: Into<S> {
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*)
            }
        }
        impl<R, S> From<(R, R)> for $name<S> where R: Into<S> {
            fn from(value: (R, R)) -> Self {
                Self{$($bit: value.$num.into()),*}
            }
        }

        hom_trait_impls!(normal_impls, $name, 2, [$($bit:$num),*], $display, $upperhex, $lowerhex);
    };
    ($name:ident, 3, [$($bit:tt:$num:tt),*], $display:literal, $upperhex:literal, $lowerhex:literal) => {
        impl<R, S> From<$name<R>> for (S, S, S) where R: Into<S> {
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*)
            }
        }
        impl<R, S> From<(R, R, R)> for $name<S> where R: Into<S> {
            fn from(value: (R, R, R)) -> Self {
                Self{$($bit: value.$num.into()),*}
            }
        }

        hom_trait_impls!(normal_impls, $name, 3, [$($bit:$num),*], $display, $upperhex, $lowerhex);
    };
    ($name:ident, 4, [$($bit:tt:$num:tt),*], $display:literal, $upperhex:literal, $lowerhex:literal) => {
        impl<R, S> From<$name<R>> for (S, S, S, S) where R: Into<S> {
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*)
            }
        }
        impl<R, S> From<(R, R, R, R)> for $name<S> where R: Into<S> {
            fn from(value: (R, R, R, R)) -> Self {
                Self{$($bit: value.$num.into()),*}
            }
        }

        hom_trait_impls!(normal_impls, $name, 4, [$($bit:$num),*], $display, $upperhex, $lowerhex);
    };

    (normal_impls, $name:ident, $length:literal, [$($bit:tt:$num:tt),*], $display:literal, $upperhex:literal, $lowerhex:literal) => {
        impl<T: fmt::Display> fmt::Display for $name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $display, $(self.$bit),*)
            }
        }
        impl<T: fmt::UpperHex> fmt::UpperHex for $name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $upperhex, $(self.$bit),*)
            }
        }
        impl<T: fmt::LowerHex> fmt::LowerHex for $name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $lowerhex, $(self.$bit),*)
            }
        }

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

        impl<T> Sum<$name<T>> for $name<T> where T: Default + Add<Output=T> {
            #[inline(always)]
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold($name::default(), Add::add)
            }
        }

        /// Allows writing `pixel + pixel`
        impl<T: Add> Add for $name<T> {
            type Output = $name<<T as Add>::Output>;

            #[inline(always)]
            fn add(self, other: $name<T>) -> Self::Output {
                Self::Output {
                    $(
                        $bit: self.$bit + other.$bit,
                    )+
                }
            }
        }
        /// Allows writing `pixel + 1`
        impl<T> Add<T> for $name<T> where
            T: Copy + Add<Output=T>
        {
            type Output = $name<T>;

            #[inline(always)]
            fn add(self, r: T) -> Self::Output {
                Self::Output {
                    $(
                        $bit: self.$bit + r,
                    )+
                }
            }
        }

        /// Allows writing `pixel += pixel`
        impl<T> AddAssign for $name<T> where
            T: Add<Output = T> + Copy
        {
            #[inline(always)]
            fn add_assign(&mut self, other: $name<T>) {
                *self = Self {
                    $(
                        $bit: self.$bit + other.$bit,
                    )+
                };
            }
        }
        /// Allows writing `pixel += 1`
        impl<T> AddAssign<T> for $name<T> where
            T: Copy + Add<Output=T>
        {
            #[inline(always)]
            fn add_assign(&mut self, r: T) {
                *self = Self {
                    $(
                        $bit: self.$bit + r,
                    )+
                };
            }
        }

        /// Allows writing `pixel - pixel`
        impl<T: Sub> Sub for $name<T> {
            type Output = $name<<T as Sub>::Output>;

            #[inline(always)]
            fn sub(self, other: Self) -> Self::Output {
                Self::Output {
                    $(
                        $bit: self.$bit - other.$bit,
                    )+
                }
            }
        }
        /// Allows writing `pixel - 1`
        impl<T> Sub<T> for $name<T> where
            T: Copy + Sub<Output=T>
        {
            type Output = $name<<T as Sub>::Output>;

            #[inline(always)]
            fn sub(self, r: T) -> Self::Output {
                Self::Output {
                    $(
                        $bit: self.$bit - r,
                    )+
                }
            }
        }

        /// Allows writing `pixel -= pixel`
        impl<T> SubAssign for $name<T> where
            T: Sub<Output = T> + Copy
        {
            #[inline(always)]
            fn sub_assign(&mut self, other: $name<T>) {
                *self = Self {
                    $(
                        $bit: self.$bit - other.$bit,
                    )+
                };
            }
        }
        /// Allows writing `pixel -= 1`
        impl<T> SubAssign<T> for $name<T> where
            T: Copy + Sub<Output=T>
        {
            #[inline(always)]
            fn sub_assign(&mut self, r: T) {
                *self = Self {
                    $(
                        $bit: self.$bit - r,
                    )+
                };
            }
        }

        /// Allows writing `pixel * pixel`
        impl<T: Mul> Mul for $name<T> {
            type Output = $name<<T as Mul>::Output>;

            #[inline(always)]
            fn mul(self, other: Self) -> Self::Output {
                Self::Output {
                    $(
                        $bit: self.$bit * other.$bit,
                    )+
                }
            }
        }
        /// Allows writing `pixel * 2`
        impl<T> Mul<T> for $name<T> where
            T: Copy + Mul<Output=T>
        {
            type Output = $name<T>;

            #[inline(always)]
            fn mul(self, r: T) -> Self::Output {
                Self::Output {
                    $(
                        $bit: self.$bit * r,
                    )+
                }
            }
        }

        /// Allows writing `pixel *= pixel`
        impl<T> MulAssign for $name<T> where
            T: Mul<Output = T> + Copy
        {
            #[inline(always)]
            fn mul_assign(&mut self, other: $name<T>) {
                *self = Self {
                    $(
                        $bit: self.$bit * other.$bit,
                    )+
                };
            }
        }
        /// Allows writing `pixel *= 2`
        impl<T> MulAssign<T> for $name<T> where
            T: Copy + Mul<Output=T>
        {
            #[inline(always)]
            fn mul_assign(&mut self, r: T) {
                *self = Self {
                    $(
                        $bit: self.$bit * r,
                    )+
                };
            }
        }

        /// Allows writing `pixel / pixel`
        impl<T: Div> Div for $name<T> {
            type Output = $name<<T as Div>::Output>;

            #[inline(always)]
            fn div(self, other: Self) -> Self::Output {
                Self::Output {
                    $(
                        $bit: self.$bit / other.$bit,
                    )+
                }
            }
        }
        /// Allows writing `pixel / 2`
        impl<T> Div<T> for $name<T> where
            T: Copy + Div<Output=T>
        {
            type Output = $name<T>;

            #[inline(always)]
            fn div(self, r: T) -> Self::Output {
                Self::Output {
                    $(
                        $bit: self.$bit / r,
                    )+
                }
            }
        }

        /// Allows writing `pixel /= pixel`
        impl<T> DivAssign for $name<T> where
            T: Div<Output = T> + Copy
        {
            #[inline(always)]
            fn div_assign(&mut self, other: $name<T>) {
                *self = Self {
                    $(
                        $bit: self.$bit / other.$bit,
                    )+
                };
            }
        }
        /// Allows writing `pixel /= 2`
        impl<T> DivAssign<T> for $name<T> where
            T: Copy + Div<Output=T>
        {
            #[inline(always)]
            fn div_assign(&mut self, r: T) {
                *self = $name {
                    $(
                        $bit: self.$bit / r,
                    )+
                };
            }
        }
    };
}

hom_trait_impls!(Rgb, 3, [r:0, g:1, b:2], "rgb({}, {}, {})", "rgb(#{:02X}{:02X}{:02X})", "rgb(#{:02x}{:02x}{:02x})");
hom_trait_impls!(Bgr, 3, [b:0, g:1, r:2], "bgr({}, {}, {})", "bgr(#{:02X}{:02X}{:02X})", "bgr(#{:02x}{:02x}{:02x})");
hom_trait_impls!(Grb, 3, [g:0, r:1, b:2], "grb({}, {}, {})", "grb(#{:02X}{:02X}{:02X})", "grb(#{:02x}{:02x}{:02x})");
hom_trait_impls!(Rgba, 4, [r:0, g:1, b:2, a:3], "rgba({}, {}, {}, {})", "rgba(#{:02X}{:02X}{:02X}{:02X})", "rgba(#{:02x}{:02x}{:02x}{:02x})");
hom_trait_impls!(Argb, 4, [a:0, r:1, g:2, b:3], "argb({}, {}, {}, {})", "argb(#{:02X}{:02X}{:02X}{:02X})", "argb(#{:02x}{:02x}{:02x}{:02x})");
hom_trait_impls!(Bgra, 4, [b:0, g:1, r:2, a:3], "bgra({}, {}, {}, {})", "bgra(#{:02X}{:02X}{:02X}{:02X})", "bgra(#{:02x}{:02x}{:02x}{:02x})");
hom_trait_impls!(Abgr, 4, [a:0, b:1, g:2, r:3], "abgr({}, {}, {}, {})", "abgr(#{:02X}{:02X}{:02X}{:02X})", "abgr(#{:02x}{:02x}{:02x}{:02x})");
#[cfg(feature = "legacy")]
hom_trait_impls!(Gray, 1, [0:0], "gray({})", "gray(#{:02X})", "gray(#{:02x})");
#[cfg(feature = "legacy")]
hom_trait_impls!(GrayAlpha, 2, [0:0, 1:1], "grayA({}, {})", "grayA(#{:02X}{:02X})", "grayA(#{:02x}{:02x})");
hom_trait_impls!(Luma, 1, [l:0], "luma({})", "luma(#{:02X})", "luma(#{:02x})");
hom_trait_impls!(LumaA, 2, [l:0, a:1], "LumaA({}, {})", "lumaA(#{:02X}{:02X})", "lumaA(#{:02x}{:02x})");
