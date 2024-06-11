use crate::*;

/// A pixel which can gain an alpha component.
pub trait WithAlpha: HomogeneousPixel {
    /// The pixel type with its alpha component.
    type WithAlpha: HomogeneousPixel;

    /// Returns the pixel type with its alpha component. If no alpha component is already contained
    /// then it is set to the maximum value.
    /// [`PixelComponent`].
    fn with_alpha(self) -> Self::WithAlpha;
}
/// A pixel which can lose its alpha component.
pub trait WithoutAlpha: HomogeneousPixel {
    /// The pixel type without its alpha component.
    type WithoutAlpha: HomogeneousPixel;

    /// Returns the pixel type without its alpha component.
    fn without_alpha(self) -> Self::WithoutAlpha;
}

macro_rules! lower_upper {
    ($lower:ident, $upper:ident, {$($color_bit:tt),*}, $alpha_bit:tt) => {
        impl<T> WithAlpha for $lower<T> where T: PixelComponent {
            type WithAlpha = $upper<T>;

            fn with_alpha(self) -> Self::WithAlpha {
                $upper {
                    $($color_bit: self.$color_bit),*,
                    $alpha_bit: <$lower<T> as HomogeneousPixel>::Component::COMPONENT_MAX,
                }
            }
        }
        impl<T> WithoutAlpha for $upper<T> where T: PixelComponent {
            type WithoutAlpha = $lower<T>;

            fn without_alpha(self) -> Self::WithoutAlpha {
                $lower {
                    $($color_bit: self.$color_bit),*
                }
            }
        }
    };
}
macro_rules! with_no_op {
    ($original:ident) => {
        impl<T> WithAlpha for $original<T>
        where
            T: PixelComponent,
        {
            type WithAlpha = $original<T>;

            fn with_alpha(self) -> Self::WithAlpha {
                self
            }
        }
    };
}
macro_rules! without_no_op {
    ($original:ident) => {
        impl<T> WithoutAlpha for $original<T>
        where
            T: PixelComponent,
        {
            type WithoutAlpha = $original<T>;

            fn without_alpha(self) -> Self::WithoutAlpha {
                self
            }
        }
    };
}

without_no_op!(Rgb);
without_no_op!(Bgr);
without_no_op!(Grb);
without_no_op!(Gray);

with_no_op!(Rgba);
with_no_op!(Argb);
with_no_op!(Bgra);
with_no_op!(Abgr);
with_no_op!(GrayA);

lower_upper!(Rgb, Rgba, {r, g, b}, a);
lower_upper!(Bgr, Bgra, {r, g, b}, a);
lower_upper!(Gray, GrayA, { 0 }, 1);
