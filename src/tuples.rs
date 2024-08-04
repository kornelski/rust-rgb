use crate::formats::gray::Gray_v08;
use crate::formats::gray::Gray_v09;
use crate::formats::gray_alpha::GrayAlpha_v08;
use crate::{Abgr, Argb, Bgr, Bgra, Grb, GrayA, Rgb, Rgba, Rgbw};

macro_rules! tuple_conversion {
    ($name:ident, 1, [$($bit:tt:$num:tt),*]) => {
        impl<R, S> From<$name<R>> for (S,) where R: Into<S> {
            #[inline]
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*,)
            }
        }

        impl<R, S> From<(R,)> for $name<S> where R: Into<S> {
            #[inline]
            fn from(value: (R,)) -> Self {
                Self { $($bit: value.$num.into()),* }
            }
        }
    };
    ($name:ident, 2, [$($bit:tt:$num:tt),*]) => {
        impl<R, S> From<$name<R>> for (S, S) where R: Into<S> {
            #[inline]
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*)
            }
        }

        impl<R, S> From<(R, R)> for $name<S> where R: Into<S> {
            #[inline]
            fn from(value: (R, R)) -> Self {
                Self { $($bit: value.$num.into()),* }
            }
        }
    };
    ($name:ident, 3, [$($bit:tt:$num:tt),*]) => {
        impl<R, S> From<$name<R>> for (S, S, S) where R: Into<S> {
            #[inline]
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*)
            }
        }

        impl<R, S> From<(R, R, R)> for $name<S> where R: Into<S> {
            #[inline]
            fn from(value: (R, R, R)) -> Self {
                Self { $($bit: value.$num.into()),* }
            }
        }
    };
    ($name:ident, 4, [$($bit:tt:$num:tt),*]) => {
        impl<R, S> From<$name<R>> for (S, S, S, S) where R: Into<S> {
            #[inline]
            fn from(value: $name<R>) -> Self {
                ($(value.$bit.into()),*)
            }
        }

        impl<R, S> From<(R, R, R, R)> for $name<S> where R: Into<S> {
            #[inline]
            fn from(value: (R, R, R, R)) -> Self {
                Self { $($bit: value.$num.into()),* }
            }
        }
    };
}

tuple_conversion!(Rgb, 3, [r:0, g:1, b:2]);
tuple_conversion!(Bgr, 3, [b:0, g:1, r:2]);
tuple_conversion!(Grb, 3, [g:0, r:1, b:2]);
tuple_conversion!(Gray_v09, 1, [v:0]);
tuple_conversion!(Gray_v08, 1, [0:0]);
tuple_conversion!(Rgbw, 4, [r:0, g:1, b:2, w:3]);

tuple_conversion!(Rgba, 4, [r:0, g:1, b:2, a:3]);
tuple_conversion!(Argb, 4, [a:0, r:1, g:2, b:3]);
tuple_conversion!(Bgra, 4, [b:0, g:1, r:2, a:3]);
tuple_conversion!(Abgr, 4, [a:0, b:1, g:2, r:3]);
tuple_conversion!(GrayA, 2, [v:0, a:1]);
tuple_conversion!(GrayAlpha_v08, 2, [0:0, 1:1]);
