use crate::HasAlpha;
use crate::HetPixel;
use crate::Luma;
use crate::LumaA;
use crate::PixelComponent;
use crate::{Abgr, Argb, Bgr, Bgra, Rgb, Rgba};
#[cfg(feature = "legacy")]
use crate::{Gray, GrayAlpha};

/// A pixel which can gain an alpha component.
pub trait GainAlpha: HetPixel {
    /// The pixel type after gaining an alpha component.
    type GainAlpha: HasAlpha;

    /// Returns the pixel type after gaining an alpha component.
    ///
    /// If an alpha is already contained then it remains at the same value. If no alpha component
    /// is already contained then it is set to the maximum value via [`PixelComponent`].
    fn gain_alpha(self) -> Self::GainAlpha;

    /// Returns the pixel type after gaining an alpha component.
    ///
    /// If an alpha is already contained then it remains at the same value. If no alpha component
    /// is already contained then it is set to the given `alpha` value.
    fn gain_alpha_with(self, alpha: Self::AlphaComponent) -> Self::GainAlpha;

    /// Returns the pixel type after gaining an alpha component.
    ///
    /// The alpha value is set to the given `alpha` value regardless of whether the pixel already
    /// contained an alpha component.
    fn gain_alpha_exact(self, alpha: Self::AlphaComponent) -> Self::GainAlpha;
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
            fn gain_alpha_with(self, alpha: Self::AlphaComponent) -> Self::GainAlpha {
                $upper {
                    $($color_bit: self.$color_bit),*,
                    $alpha_bit: alpha,
                }
            }
            fn gain_alpha_exact(self, alpha: Self::AlphaComponent) -> Self::GainAlpha {
                $upper {
                    $($color_bit: self.$color_bit),*,
                    $alpha_bit: alpha,
                }
            }
        }
    };
}
macro_rules! gain_already_alpha {
    ($original:ident, $alpha_bit: tt) => {
        impl<T> GainAlpha for $original<T>
        where
            T: PixelComponent,
        {
            type GainAlpha = $original<T>;

            fn gain_alpha(self) -> Self::GainAlpha {
                self
            }
            fn gain_alpha_with(self, _: Self::AlphaComponent) -> Self::GainAlpha {
                self
            }
            fn gain_alpha_exact(mut self, alpha: Self::AlphaComponent) -> Self::GainAlpha {
                self.$alpha_bit = alpha;
                self
            }
        }
    };
}

gain_already_alpha!(Rgba, a);
gain_already_alpha!(Argb, a);
gain_already_alpha!(Bgra, a);
gain_already_alpha!(Abgr, a);
#[cfg(feature = "legacy")]
gain_already_alpha!(GrayAlpha, 1);
gain_already_alpha!(LumaA, a);

lower_upper!(Rgb, Rgba, {r, g, b}, a);
lower_upper!(Bgr, Bgra, {r, g, b}, a);
#[cfg(feature = "legacy")]
lower_upper!(Gray, GrayAlpha, { 0 }, 1);
lower_upper!(Luma, LumaA, { l }, a);
