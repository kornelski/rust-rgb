pub mod abgr;
pub mod argb;
pub mod bgr;
pub mod bgra;
pub mod gray;
pub mod gray_a;
#[cfg(feature = "legacy")]
pub mod gray_alpha;
pub mod grb;
pub mod rgb;
pub mod rgba;
pub mod rgbw;

use crate::{Abgr, Argb, Bgr, Bgra, Grb, Gray, GrayA, Rgb, Rgba, Rgbw};
use core::array::TryFromSliceError;
use core::fmt;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! tuple_conversion {
    ($name:ident, 1, [$($bit:tt:$num:tt),*]) => {
    };
    ($name:ident, 2, [$($bit:tt:$num:tt),*]) => {
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
    };
    ($name:ident, 3, [$($bit:tt:$num:tt),*]) => {
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
    };
    ($name:ident, 4, [$($bit:tt:$num:tt),*]) => {
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
    };
}

macro_rules! trait_impls_with_alpha {
    ($name:ident, $length:literal, [$($bit:tt),*], $display:literal, $upperhex:literal, $lowerhex:literal) => {
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
            T: Copy + 'static,
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

        impl<T, A> Sum<$name<T, A>> for $name<T, A> where T: Default + Add<Output=T>, A: Default + Add<Output=A> {
            #[inline(always)]
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold($name::default(), Add::add)
            }
        }

        /// Allows writing `pixel + pixel`
        impl<T: Add, A: Add> Add for $name<T, A> {
            type Output = $name<T::Output, A::Output>;

            #[inline(always)]
            fn add(self, other: Self) -> Self::Output {
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
        impl<T, A> AddAssign for $name<T, A> where
            T: Add<Output = T> + Copy,
            A: Add<Output = A> + Copy,
        {
            #[inline(always)]
            fn add_assign(&mut self, other: Self) {
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
        impl<T: Sub, A: Sub> Sub for $name<T, A> {
            type Output = $name<T::Output, A::Output>;

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
            type Output = $name<T::Output>;

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
        impl<T, A> SubAssign for $name<T, A> where
            T: Sub<Output = T> + Copy,
            A: Sub<Output = A> + Copy,
        {
            #[inline(always)]
            fn sub_assign(&mut self, other: Self) {
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
        impl<T: Mul, A: Mul> Mul for $name<T, A> {
            type Output = $name<T::Output, A::Output>;

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
        impl<T, A> MulAssign for $name<T, A> where
            T: Mul<Output = T> + Copy,
            A: Mul<Output = A> + Copy,
        {
            #[inline(always)]
            fn mul_assign(&mut self, other: Self) {
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
        impl<T: Div, A: Div> Div for $name<T, A> {
            type Output = $name<T::Output, A::Output>;

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
        impl<T, A> DivAssign for $name<T, A> where
            T: Div<Output = T> + Copy,
            A: Div<Output = A> + Copy,
        {
            #[inline(always)]
            fn div_assign(&mut self, other: Self) {
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

macro_rules! trait_impls_without_alpha {
    ($name:ident, $length:literal, [$($bit:tt),*], $display:literal, $upperhex:literal, $lowerhex:literal) => {
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
            T: Copy + 'static,
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
            type Output = $name<T::Output>;

            #[inline(always)]
            fn add(self, other: Self) -> Self::Output {
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
            fn add_assign(&mut self, other: Self) {
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
            type Output = $name<T::Output>;

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
            type Output = $name<T::Output>;

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
            fn sub_assign(&mut self, other: Self) {
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
            type Output = $name<T::Output>;

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
            fn mul_assign(&mut self, other: Self) {
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
            type Output = $name<T::Output>;

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
            fn div_assign(&mut self, other: Self) {
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

tuple_conversion!(Rgb, 3, [r:0, g:1, b:2]);
tuple_conversion!(Bgr, 3, [b:0, g:1, r:2]);
tuple_conversion!(Grb, 3, [g:0, r:1, b:2]);
tuple_conversion!(Gray, 1, [v:0]);
tuple_conversion!(Rgbw, 4, [r:0, g:1, b:2, w:3]);

tuple_conversion!(Rgba, 4, [r:0, g:1, b:2, a:3]);
tuple_conversion!(Argb, 4, [a:0, r:1, g:2, b:3]);
tuple_conversion!(Bgra, 4, [b:0, g:1, r:2, a:3]);
tuple_conversion!(Abgr, 4, [a:0, b:1, g:2, r:3]);
tuple_conversion!(GrayA, 2, [v:0, a:1]);

trait_impls_without_alpha!(Rgb, 3, [r, g, b], "rgb({}, {}, {})", "rgb(#{:02X}{:02X}{:02X})", "rgb(#{:02x}{:02x}{:02x})");
trait_impls_without_alpha!(Bgr, 3, [b, g, r], "bgr({}, {}, {})", "bgr(#{:02X}{:02X}{:02X})", "bgr(#{:02x}{:02x}{:02x})");
trait_impls_without_alpha!(Grb, 3, [g, r, b], "grb({}, {}, {})", "grb(#{:02X}{:02X}{:02X})", "grb(#{:02x}{:02x}{:02x})");
trait_impls_without_alpha!(Gray, 1, [v], "gray({})", "gray(#{:02X})", "gray(#{:02x})");
trait_impls_without_alpha!(Rgbw, 4, [r, g, b, w], "rgbw({}, {}, {}, {})", "rgbw(#{:02X}{:02X}{:02X}{:02X})", "rgbw(#{:02x}{:02x}{:02x}{:02x})");

trait_impls_with_alpha!(Rgba, 4, [r, g, b, a], "rgba({}, {}, {}, {})", "rgba(#{:02X}{:02X}{:02X}{:02X})", "rgba(#{:02x}{:02x}{:02x}{:02x})");
trait_impls_with_alpha!(Argb, 4, [a, r, g, b], "argb({}, {}, {}, {})", "argb(#{:02X}{:02X}{:02X}{:02X})", "argb(#{:02x}{:02x}{:02x}{:02x})");
trait_impls_with_alpha!(Bgra, 4, [b, g, r, a], "bgra({}, {}, {}, {})", "bgra(#{:02X}{:02X}{:02X}{:02X})", "bgra(#{:02x}{:02x}{:02x}{:02x})");
trait_impls_with_alpha!(Abgr, 4, [a, b, g, r], "abgr({}, {}, {}, {})", "abgr(#{:02X}{:02X}{:02X}{:02X})", "abgr(#{:02x}{:02x}{:02x}{:02x})");
trait_impls_with_alpha!(GrayA, 2, [v, a], "graya({}, {})", "graya(#{:02X}{:02X})", "graya(#{:02x}{:02x})");
