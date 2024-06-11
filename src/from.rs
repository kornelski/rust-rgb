use crate::*;

macro_rules! between {
    ($from_type:ident, $self_type:ident, {$($bit:tt),*}) => {
        impl<R, S> From<$from_type<R>> for $self_type<S> where R: Into<S> {
            fn from(value: $from_type<R>) -> Self {
                Self{$($bit: value.$bit.into()),*}
            }
        }
    };
}

between!(Rgba, Rgb, {r, g, b});
between!(Argb, Rgb, {r, g, b});
between!(Bgra, Rgb, {r, g, b});
between!(Abgr, Rgb, {r, g, b});
between!(Bgr, Rgb, {r, g, b});
between!(Grb, Rgb, {r, g, b});

between!(Rgba, Bgr, {r, g, b});
between!(Argb, Bgr, {r, g, b});
between!(Bgra, Bgr, {r, g, b});
between!(Abgr, Bgr, {r, g, b});
between!(Rgb, Bgr, {r, g, b});
between!(Grb, Bgr, {r, g, b});

between!(Rgba, Grb, {r, g, b});
between!(Argb, Grb, {r, g, b});
between!(Bgra, Grb, {r, g, b});
between!(Abgr, Grb, {r, g, b});
between!(Rgb, Grb, {r, g, b});
between!(Bgr, Grb, {r, g, b});

between!(Argb, Rgba, {r, g, b, a});
between!(Bgra, Rgba, {r, g, b, a});
between!(Abgr, Rgba, {r, g, b, a});

between!(Rgba, Argb, {r, g, b, a});
between!(Bgra, Argb, {r, g, b, a});
between!(Abgr, Argb, {r, g, b, a});

between!(Rgba, Bgra, {r, g, b, a});
between!(Argb, Bgra, {r, g, b, a});
between!(Abgr, Bgra, {r, g, b, a});

between!(Rgba, Abgr, {r, g, b, a});
between!(Argb, Abgr, {r, g, b, a});
between!(Bgra, Abgr, {r, g, b, a});

between!(GrayA, Gray, { 0 });

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
