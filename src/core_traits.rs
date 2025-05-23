use crate::{Abgr, Argb, Bgr, Bgra, GrayA, Gray_v09, Grb, Rgb, Rgba, Rgbw};
use core::array::TryFromSliceError;
use core::fmt;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! trait_impls_common {
    ($name:ident, $length:literal, [$($bit:tt),*]) => {
        impl<R, S> From<$name<R>> for [S; $length] where R: Into<S> {
            #[inline]
            fn from(value: $name<R>) -> Self {
                [$(value.$bit.into()),*]
            }
        }

        impl<R, S> From<[R; $length]> for $name<S> where R: Into<S> {
            #[inline]
            fn from(value: [R; $length]) -> Self {
                let mut iter = value.into_iter();
                Self { $($bit: iter.next().unwrap().into()),* }
            }
        }

        impl<T> TryFrom<&[T]> for $name<T> where T: Copy + 'static {
            type Error = TryFromSliceError;

            #[inline]
            fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
                let array: [T; $length] = slice.try_into()?;
                Ok(Self::from(array))
            }
        }

        impl<T> IntoIterator for $name<T> {
            type Item = T;
            type IntoIter = core::array::IntoIter<T, $length>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                [$(self.$bit.into()),*].into_iter()
            }
        }

        /// Allows writing `pixel + 1`
        impl<T> Add<T> for $name<T> where T: Copy + Add<Output=T> {
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

        /// Allows writing `pixel += 1`
        impl<T> AddAssign<T> for $name<T> where T: Copy + Add<Output=T> {
            #[inline(always)]
            fn add_assign(&mut self, r: T) {
                *self = Self {
                    $(
                        $bit: self.$bit + r,
                    )+
                };
            }
        }

        /// Allows writing `pixel - 1`
        impl<T> Sub<T> for $name<T> where T: Copy + Sub<Output=T> {
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

        /// Allows writing `pixel -= 1`
        impl<T> SubAssign<T> for $name<T> where T: Copy + Sub<Output=T> {
            #[inline(always)]
            fn sub_assign(&mut self, r: T) {
                *self = Self {
                    $(
                        $bit: self.$bit - r,
                    )+
                };
            }
        }

        /// Allows writing `pixel * 2`
        impl<T> Mul<T> for $name<T> where T: Copy + Mul<Output=T> {
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

        /// Allows writing `pixel *= 2`
        impl<T> MulAssign<T> for $name<T> where T: Copy + Mul<Output=T> {
            #[inline(always)]
            fn mul_assign(&mut self, r: T) {
                *self = Self {
                    $(
                        $bit: self.$bit * r,
                    )+
                };
            }
        }

        /// Allows writing `pixel / 2`
        impl<T> Div<T> for $name<T> where T: Copy + Div<Output=T> {
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

        /// Allows writing `pixel /= 2`
        impl<T> DivAssign<T> for $name<T> where T: Copy + Div<Output=T> {
            #[inline(always)]
            fn div_assign(&mut self, r: T) {
                *self = $name {
                    $(
                        $bit: self.$bit / r,
                    )+
                };
            }
        }

        impl<T> AsRef<[T; $length]> for $name<T> where T: Copy + 'static {
            #[inline]
            fn as_ref(&self) -> &[T; $length] {
                crate::Pixel::as_array(self)
            }
        }

        impl<T> AsMut<[T; $length]> for $name<T> where T: Copy + 'static {
            #[inline]
            fn as_mut(&mut self) -> &mut [T; $length] {
                crate::Pixel::as_array_mut(self)
            }
        }
    };
}

macro_rules! trait_impls_with_alpha {
    ($name:ident, $length:literal, [$($printas:ident => $bit:tt),*], $display:literal, $upperhex:literal, $lowerhex:literal) => {
        trait_impls_common!($name, $length, [$($bit),*]);

        impl<T: fmt::Display, A: fmt::Display> fmt::Display for $name<T, A> {
            #[cold]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $display, $($printas = self.$bit),*)
            }
        }

        impl<T: fmt::UpperHex, A: fmt::UpperHex> fmt::UpperHex for $name<T, A> {
            #[cold]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let width = 2 * core::mem::size_of::<T>();
                write!(f, $upperhex, $($printas = self.$bit),* , w = width)
            }
        }

        impl<T: fmt::LowerHex, A: fmt::LowerHex> fmt::LowerHex for $name<T, A> {
            #[cold]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let width = 2 * core::mem::size_of::<T>();
                write!(f, $lowerhex, $($printas = self.$bit),* , w = width)
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
    };
}

macro_rules! trait_impls_without_alpha {
    ($name:ident, $length:literal, [$($printas:ident => $bit:tt),*], $display:literal, $upperhex:literal, $lowerhex:literal) => {
        trait_impls_common!($name, $length, [$($bit),*]);

        impl<T: fmt::Display> fmt::Display for $name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $display, $($printas = self.$bit),*)
            }
        }

        impl<T: fmt::UpperHex> fmt::UpperHex for $name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let width = 2 * core::mem::size_of::<T>();
                write!(f, $upperhex, $($printas = self.$bit),*, w = width)
            }
        }

        impl<T: fmt::LowerHex> fmt::LowerHex for $name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let width = 2 * core::mem::size_of::<T>();
                write!(f, $lowerhex, $($printas = self.$bit),*, w = width)
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

        /// Allows writing `pixel += pixel`
        impl<T> AddAssign for $name<T> where T: Add<Output = T> + Copy {
            #[inline(always)]
            fn add_assign(&mut self, other: Self) {
                *self = Self {
                    $(
                        $bit: self.$bit + other.$bit,
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

        /// Allows writing `pixel -= pixel`
        impl<T> SubAssign for $name<T> where T: Sub<Output = T> + Copy {
            #[inline(always)]
            fn sub_assign(&mut self, other: Self) {
                *self = Self {
                    $(
                        $bit: self.$bit - other.$bit,
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

        /// Allows writing `pixel *= pixel`
        impl<T> MulAssign for $name<T> where T: Mul<Output = T> + Copy {
            #[inline(always)]
            fn mul_assign(&mut self, other: Self) {
                *self = Self {
                    $(
                        $bit: self.$bit * other.$bit,
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

        /// Allows writing `pixel /= pixel`
        impl<T> DivAssign for $name<T> where T: Div<Output = T> + Copy {
            #[inline(always)]
            fn div_assign(&mut self, other: Self) {
                *self = Self {
                    $(
                        $bit: self.$bit / other.$bit,
                    )+
                };
            }
        }
    };
}

trait_impls_without_alpha!(Rgb, 3, [r => r, g => g, b => b], "rgb({r},{g},{b})", "#{r:0w$X}{g:0w$X}{b:0w$X}", "#{r:0w$x}{g:0w$x}{b:0w$x}");
trait_impls_without_alpha!(Bgr, 3, [b => b, g => g, r => r], "bgr({b},{g},{r})", "#{r:0w$X}{g:0w$X}{b:0w$X}", "#{r:0w$x}{g:0w$x}{b:0w$x}");
trait_impls_without_alpha!(Grb, 3, [g => g, r => r, b => b], "grb({g},{r},{b})", "#{r:0w$X}{g:0w$X}{b:0w$X}", "#{r:0w$x}{g:0w$x}{b:0w$x}");
trait_impls_without_alpha!(Gray_v09, 1, [v => v], "gray({v})", "gray(#{v:0w$X})", "gray(#{v:0w$x})");
trait_impls_without_alpha!(Rgbw, 4, [r => r, g => g, b => b, white => w], "rgbw({r},{g},{b},{white})", "#{r:0w$X}{g:0w$X}{b:0w$X}{white:0w$X}", "#{r:0w$x}{g:0w$x}{b:0w$x}{white:0w$x}");

use crate::formats::gray::Gray_v08;
trait_impls_without_alpha!(Gray_v08, 1, [v => 0], "gray_v0.8({v})", "gray_v0.8(#{v:0w$X})", "gray_v0.8(#{v:0w$x})");

trait_impls_with_alpha!(Rgba, 4, [r => r, g => g, b => b, a => a], "rgba({r},{g},{b},{a})", "#{r:0w$X}{g:0w$X}{b:0w$X}{a:0w$X}", "#{r:0w$x}{g:0w$x}{b:0w$x}{a:0w$x}");
trait_impls_with_alpha!(Argb, 4, [a => a, r => r, g => g, b => b], "argb({a},{r},{g},{b})", "#{r:0w$X}{g:0w$X}{b:0w$X}{a:0w$X}", "#{r:0w$x}{g:0w$x}{b:0w$x}{a:0w$x}");
trait_impls_with_alpha!(Bgra, 4, [b => b, g => g, r => r, a => a], "bgra({b},{g},{r},{a})", "#{r:0w$X}{g:0w$X}{b:0w$X}{a:0w$X}", "#{r:0w$x}{g:0w$x}{b:0w$x}{a:0w$x}");
trait_impls_with_alpha!(Abgr, 4, [a => a, b => b, g => g, r => r], "abgr({a},{b},{g},{r})", "#{r:0w$X}{g:0w$X}{b:0w$X}{a:0w$X}", "#{r:0w$x}{g:0w$x}{b:0w$x}{a:0w$x}");
trait_impls_with_alpha!(GrayA, 2, [v => v, a => a], "graya({v},{a})", "graya(#{v:0w$X}{a:0w$X})", "graya(#{v:0w$x}{a:0w$x})");

use crate::formats::gray_alpha::GrayAlpha_v08;
trait_impls_with_alpha!(GrayAlpha_v08, 2, [v => 0, a => 1], "graya_v0.8({v},{a})", "graya_v0.8(#{v:0w$X}{a:0w$X})", "graya_v0.8(#{v:0w$x}{a:0w$x})");

#[cfg(test)]
#[test]
fn test_16_fmt() {
    extern crate std;

    let a = Argb::<u16>::new_argb(1, 0x1234, 3, 65535);
    assert_eq!("#12340003FFFF0001", &std::format!("{a:X}"));
}

#[test]
#[allow(deprecated)]
fn convert_array() {
    use crate::{BGR8, BGRA8, RGB8, RGBA8};

    assert_eq!(RGB8::from([1, 2, 3]), RGB8::new(1, 2, 3));
    assert_eq!(Into::<[u8; 3]>::into(RGB8::new(1, 2, 3)), [1, 2, 3]);
    assert_eq!(RGBA8::from([1, 2, 3, 4]), RGBA8::new(1, 2, 3, 4));
    assert_eq!(Into::<[u8; 4]>::into(RGBA8::new(1, 2, 3, 4)), [1, 2, 3, 4]);
    assert_eq!(BGR8::from([3, 2, 1]), BGR8::new_bgr(3, 2, 1));
    assert_eq!(Into::<[u8; 3]>::into(BGR8::new_bgr(3, 2, 1)), [3, 2, 1]);
    assert_eq!(BGRA8::from([4, 3, 2, 1]), BGRA8::new_bgra(4, 3, 2, 1));
    assert_eq!(Into::<[u8; 4]>::into(BGRA8::new_bgra(4, 3, 2, 1)), [4, 3, 2, 1]);
}
