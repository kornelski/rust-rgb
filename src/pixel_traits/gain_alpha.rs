use crate::HasAlpha;
use crate::HetPixel;
use crate::{Abgr, Argb, Bgr, Bgra, Gray, GrayA, Rgb, Rgba};

/// A pixel which can gain an alpha component.
///
/// It's implemented for every pixel type in the crate, including those which
/// already have an alpha component.
pub trait GainAlpha: HetPixel {
    /// The pixel type after gaining an alpha component.
    /// 
    /// For example, for `Rgb`: `GainAlpha = Rgba`.
    type GainAlpha: HasAlpha;

    /// Returns the pixel type after gaining an alpha component.
    ///
    /// If an alpha is already contained then it remains at the same value. If
    /// no alpha component is already contained then it is set to the given
    /// `alpha` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Rgb, Rgba, GainAlpha};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// assert_eq!(rgb.with_default_alpha(0), Rgba {r: 0, g: 10, b: 100, a: 0});
    /// assert_eq!(rgba.with_default_alpha(0), Rgba {r: 0, g: 10, b: 100, a: 50});
    /// ```
    #[doc(alias = "gain_alpha")]
    fn with_default_alpha(self, alpha: Self::AlphaComponent) -> Self::GainAlpha;

    /// Returns the pixel type after gaining an alpha component.
    ///
    /// The alpha value is set to the given `alpha` value regardless of whether
    /// the pixel already contained an alpha component.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Rgb, Rgba, GainAlpha};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// assert_eq!(rgb.with_alpha(0), Rgba {r: 0, g: 10, b: 100, a: 0});
    /// assert_eq!(rgba.with_alpha(0), Rgba {r: 0, g: 10, b: 100, a: 0});
    /// ```
    #[doc(alias = "gain_alpha")]
    fn with_alpha(self, alpha: Self::AlphaComponent) -> Self::GainAlpha;
}

macro_rules! lower_upper {
    ($lower:ident, $upper:ident, {$($color_bit:tt),*}, $alpha_bit:tt) => {
        impl<T> GainAlpha for $lower<T> where T: Copy + 'static {
            type GainAlpha = $upper<T>;

            #[inline]
            fn with_default_alpha(self, alpha: Self::AlphaComponent) -> Self::GainAlpha {
                $upper {
                    $($color_bit: self.$color_bit),*,
                    $alpha_bit: alpha,
                }
            }

            #[inline]
            fn with_alpha(self, alpha: Self::AlphaComponent) -> Self::GainAlpha {
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
        impl<T> GainAlpha for $original<T> where T: Copy + 'static {
            type GainAlpha = $original<T>;

            #[inline]
            fn with_default_alpha(self, _: Self::AlphaComponent) -> Self::GainAlpha {
                self
            }

            #[inline]
            fn with_alpha(mut self, alpha: Self::AlphaComponent) -> Self::GainAlpha {
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
gain_already_alpha!(GrayA, a);

lower_upper!(Rgb, Rgba, {r, g, b}, a);
lower_upper!(Bgr, Bgra, {r, g, b}, a);
lower_upper!(Gray, GrayA, { v }, a);
