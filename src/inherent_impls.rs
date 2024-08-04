use crate::{Abgr, Argb, Bgr, Bgra, Gray, GrayA, Grb, Rgb, Rgba, Rgbw};

macro_rules! inherent_impls {
    ($name:ident, $new_fn:ident, [$($field:tt $var:ident),*]) => {
        impl<T: Copy + 'static> $name<T> {
            #[doc=concat!("Creates a new [`", stringify!($name), "`] pixel type from its components.")]
            ///
            /// Alternatively, you can use struct literal syntax to
            /// create the new pixel type:
            ///```not_rust
            #[doc=concat!("use rgb::", stringify!($name), ";")]
            ///
            #[doc=concat!("let pixel = ", stringify!($name), " {", stringify!($($field: $var),*), "};")]
            ///```
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
inherent_impls!(Rgbw, new, [r red, g green, b blue, w white]);

use crate::formats::gray::Gray_v08;
inherent_impls!(Gray_v08, new, [0 value]);

inherent_impls!(Rgba, new, [r red, g green, b blue, a alpha]);
inherent_impls!(Argb, new_argb, [a alpha, r red, g green, b blue]);
inherent_impls!(Bgra, new_bgra, [b blue, g green, r red, a alpha]);
inherent_impls!(Abgr, new_abgr, [a alpha, b blue, g green, r red]);
inherent_impls!(GrayA, new, [v value, a alpha]);

use crate::formats::gray_alpha::GrayAlpha_v08;
inherent_impls!(GrayAlpha_v08, new, [0 value, 1 alpha]);
