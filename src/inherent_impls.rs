
use crate::{Abgr, Argb, Bgr, Bgra, Grb, Gray, GrayA, Rgb, Rgba, Rgbw};

macro_rules! inherent_impls {
    ($name:ident, $new_fn:ident, [$($field:tt $var:ident),*]) => {
        impl<T: Copy + 'static> $name<T> {
            #[doc=concat!("`", stringify!($name))]
            /// {
            #[doc=stringify!($($field,)*)]
            /// }`
            pub const fn $new_fn($($var: T),*) -> Self {
                Self {$($field: $var),*}
            }
        }
    }
}

inherent_impls!(Rgb, new, [r red, g green, b blue]);
inherent_impls!(Bgr, new_bgr, [b blue, g green, r red]);
inherent_impls!(Grb, new_grb, [g green, r red, b blue]);
inherent_impls!(Gray, new, [v value]);
inherent_impls!(Rgbw, new_rgbw, [r red, g green, b blue, w white]);

#[cfg(feature = "legacy")]
use crate::formats::gray::Gray_v08;
#[cfg(feature = "legacy")]
inherent_impls!(Gray_v08, new, [0 value]);

inherent_impls!(Rgba, new, [r red, g green, b blue, a alpha]);
inherent_impls!(Argb, new_argb, [a alpha, r red, g green, b blue]);
inherent_impls!(Bgra, new_bgra, [b blue, g green, r red, a alpha]);
inherent_impls!(Abgr, new_abgr, [a alpha, b blue, g green, r red]);
inherent_impls!(GrayA, new, [v value, a alpha]);

#[cfg(feature = "legacy")]
use crate::formats::gray_alpha::GrayAlpha_v08;
#[cfg(feature = "legacy")]
inherent_impls!(GrayAlpha_v08, new, [0 value, 1 alpha]);
