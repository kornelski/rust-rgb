use crate::{Abgr, Argb, Bgr, Bgra, Grb, Luma, LumaA, Rgb, Rgba};
#[cfg(feature = "legacy")]
use crate::{Gray, GrayAlpha};

macro_rules! without_alpha {
    ($from_type:ident, $self_type:ident, {$($bit:tt),*}) => {
        impl<T> From<$from_type<T>> for $self_type<T> {
            fn from(value: $from_type<T>) -> Self {
                Self{$($bit: value.$bit.into()),*}
            }
        }
    };
}
macro_rules! with_alpha {
    ($from_type:ident, $self_type:ident, {$($bit:tt),*}) => {
        impl<T, A> From<$from_type<T, A>> for $self_type<T, A> {
            fn from(value: $from_type<T, A>) -> Self {
                Self{$($bit: value.$bit.into()),*}
            }
        }
    };
}
macro_rules! alpha_to_no_alpha {
    ($from_type:ident, $self_type:ident, {$($bit:tt),*}) => {
        impl<T, A> From<$from_type<T, A>> for $self_type<T> {
            fn from(value: $from_type<T, A>) -> Self {
                Self{$($bit: value.$bit.into()),*}
            }
        }
    };
}
#[cfg(feature = "legacy")]
macro_rules! alpha_to_no_alpha_verbose {
    ($from_type:ident, $self_type:ident, {$($from_bit:tt:$self_bit:tt),*}) => {
        impl<T, A> From<$from_type<T, A>> for $self_type<T> {
            fn from(value: $from_type<T, A>) -> Self {
                Self{$($self_bit: value.$from_bit.into()),*}
            }
        }
    };
}

alpha_to_no_alpha!(Rgba, Rgb, {r, g, b});
alpha_to_no_alpha!(Argb, Rgb, {r, g, b});
alpha_to_no_alpha!(Bgra, Rgb, {r, g, b});
alpha_to_no_alpha!(Abgr, Rgb, {r, g, b});
without_alpha!(Bgr, Rgb, {r, g, b});
without_alpha!(Grb, Rgb, {r, g, b});

alpha_to_no_alpha!(Rgba, Bgr, {r, g, b});
alpha_to_no_alpha!(Argb, Bgr, {r, g, b});
alpha_to_no_alpha!(Bgra, Bgr, {r, g, b});
alpha_to_no_alpha!(Abgr, Bgr, {r, g, b});
without_alpha!(Rgb, Bgr, {r, g, b});
without_alpha!(Grb, Bgr, {r, g, b});

alpha_to_no_alpha!(Rgba, Grb, {r, g, b});
alpha_to_no_alpha!(Argb, Grb, {r, g, b});
alpha_to_no_alpha!(Bgra, Grb, {r, g, b});
alpha_to_no_alpha!(Abgr, Grb, {r, g, b});
without_alpha!(Rgb, Grb, {r, g, b});
without_alpha!(Bgr, Grb, {r, g, b});

with_alpha!(Argb, Rgba, {r, g, b, a});
with_alpha!(Bgra, Rgba, {r, g, b, a});
with_alpha!(Abgr, Rgba, {r, g, b, a});

with_alpha!(Rgba, Argb, {r, g, b, a});
with_alpha!(Bgra, Argb, {r, g, b, a});
with_alpha!(Abgr, Argb, {r, g, b, a});

with_alpha!(Rgba, Bgra, {r, g, b, a});
with_alpha!(Argb, Bgra, {r, g, b, a});
with_alpha!(Abgr, Bgra, {r, g, b, a});

with_alpha!(Rgba, Abgr, {r, g, b, a});
with_alpha!(Argb, Abgr, {r, g, b, a});
with_alpha!(Bgra, Abgr, {r, g, b, a});

#[cfg(feature = "legacy")]
alpha_to_no_alpha!(GrayAlpha, Gray, { 0 });
#[cfg(feature = "legacy")]
alpha_to_no_alpha_verbose!(LumaA, Gray, { l:0 });

alpha_to_no_alpha!(LumaA, Luma, { l });
#[cfg(feature = "legacy")]
alpha_to_no_alpha_verbose!(GrayAlpha, Luma, { 0:l });

#[cfg(feature = "legacy")]
impl<R, S, T, U> From<LumaA<R, S>> for GrayAlpha<T, U>
where
    R: Into<T>,
    S: Into<U>,
{
    fn from(value: LumaA<R, S>) -> Self {
        Self(value.l.into(), value.a.into())
    }
}
#[cfg(feature = "legacy")]
impl<R, S, T, U> From<GrayAlpha<R, S>> for LumaA<T, U>
where
    R: Into<T>,
    S: Into<U>,
{
    fn from(value: GrayAlpha<R, S>) -> Self {
        Self {
            l: value.0.into(),
            a: value.1.into(),
        }
    }
}
#[cfg(feature = "legacy")]
impl<R, S> From<Luma<R>> for Gray<S>
where
    R: Into<S>,
{
    fn from(value: Luma<R>) -> Self {
        Self(value.l.into())
    }
}
#[cfg(feature = "legacy")]
impl<R, S> From<Gray<R>> for Luma<S>
where
    R: Into<S>,
{
    fn from(value: Gray<R>) -> Self {
        Self { l: value.0.into() }
    }
}
