use crate::*;

macro_rules! without_alpha {
    ($from_type:ident, $self_type:ident, {$($bit:tt),*}) => {
        impl<R, S> From<$from_type<R>> for $self_type<S> where R: Into<S> {
            fn from(value: $from_type<R>) -> Self {
                Self{$($bit: value.$bit.into()),*}
            }
        }
    };
}
macro_rules! with_alpha {
    ($from_type:ident, $self_type:ident, {$($bit:tt),*}) => {
        impl<R, S, T, U> From<$from_type<R, S>> for $self_type<T, U> where R: Into<T>, S: Into<U> {
            fn from(value: $from_type<R, S>) -> Self {
                Self{$($bit: value.$bit.into()),*}
            }
        }
    };
}
macro_rules! alpha_to_no_alpha {
    ($from_type:ident, $self_type:ident, {$($bit:tt),*}) => {
        impl<R, S, T> From<$from_type<R, S>> for $self_type<T> where R: Into<T> {
            fn from(value: $from_type<R, S>) -> Self {
                Self{$($bit: value.$bit.into()),*}
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

alpha_to_no_alpha!(GrayA, Gray, { 0 });

macro_rules! with_array {
    ($type:ident, $length:literal, [$($bit:tt),*]) => {
        impl<R, S> From<$type<R>> for [S; $length] where R: Into<S> {
            fn from(value: $type<R>) -> Self {
                [$(value.$bit.into()),*]
            }
        }
        impl<R, S> From<[R; $length]> for $type<S> where R: Into<S> {
            fn from(value: [R; $length]) -> Self {
                let mut iter = value.into_iter();
                Self{$($bit: iter.next().unwrap().into()),*}
            }
        }
    };
}

with_array!(Rgb, 3, [r, g, b]);
with_array!(Bgr, 3, [b, g, r]);
with_array!(Grb, 3, [g, r, b]);
with_array!(Rgba, 4, [r, g, b, a]);
with_array!(Argb, 4, [a, r, g, b]);
with_array!(Bgra, 4, [b, g, r, a]);
with_array!(Abgr, 4, [a, b, g, r]);
with_array!(Gray, 1, [0]);
with_array!(GrayA, 2, [0, 1]);
