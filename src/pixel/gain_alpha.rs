use crate::HetPixel;
use crate::PixelComponent;
use crate::{Abgr, Argb, Bgr, Bgra, Gray, GrayA, Grb, Rgb, Rgba};

/// A pixel which can gain an alpha component.
pub trait GainAlpha: HetPixel {
    /// The pixel type after gaining an alpha component.
    type GainAlpha: HetPixel;

    /// Returns the pixel type after gaining an alpha component.
    ///
    /// If an alpha is already contained then it remains at the same value. If no alpha component
    /// is already contained then it is set to the maximum value via [`PixelComponent`].
    fn gain_alpha(self) -> Self::GainAlpha;
}
/// A pixel which can lose its alpha component.
pub trait LoseAlpha: HetPixel {
    /// The pixel type after losing its alpha component.
    type LoseAlpha: HetPixel;

    /// Returns the pixel type after losing its alpha component.
    fn lose_alpha(self) -> Self::LoseAlpha;
}

macro_rules! lower_upper {
    ($lower:ident, $upper:ident, {$($color_bit:tt),*}, $alpha_bit:tt) => {
        impl<T> GainAlpha for $lower<T> where T: PixelComponent {
            type GainAlpha = $upper<T>;

            fn gain_alpha(self) -> Self::GainAlpha {
                $upper {
                    $($color_bit: self.$color_bit),*,
                    $alpha_bit: <$lower<T> as HetPixel>::AlphaComponent::COMPONENT_MAX,
                }
            }
        }
        impl<T> LoseAlpha for $upper<T> where T: PixelComponent {
            type LoseAlpha = $lower<T>;

            fn lose_alpha(self) -> Self::LoseAlpha {
                $lower {
                    $($color_bit: self.$color_bit),*
                }
            }
        }
    };
}
macro_rules! gain_no_op {
    ($original:ident) => {
        impl<T> GainAlpha for $original<T>
        where
            T: PixelComponent,
        {
            type GainAlpha = $original<T>;

            fn gain_alpha(self) -> Self::GainAlpha {
                self
            }
        }
    };
}
macro_rules! lose_no_op {
    ($original:ident) => {
        impl<T> LoseAlpha for $original<T>
        where
            T: PixelComponent,
        {
            type LoseAlpha = $original<T>;

            fn lose_alpha(self) -> Self::LoseAlpha {
                self
            }
        }
    };
}

lose_no_op!(Rgb);
lose_no_op!(Bgr);
lose_no_op!(Grb);
lose_no_op!(Gray);

gain_no_op!(Rgba);
gain_no_op!(Argb);
gain_no_op!(Bgra);
gain_no_op!(Abgr);
gain_no_op!(GrayA);

lower_upper!(Rgb, Rgba, {r, g, b}, a);
lower_upper!(Bgr, Bgra, {r, g, b}, a);
lower_upper!(Gray, GrayA, { 0 }, 1);
