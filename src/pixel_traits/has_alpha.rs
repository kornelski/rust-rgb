use crate::HetPixel;
use crate::{Abgr, Argb, Bgra, GrayA, Rgba};

/// A pixel which has an alpha component.
///
/// This trait is implemented only for those types in the crate that
/// contain an alpha component, such as [`Rgba`].
pub trait HasAlpha: HetPixel {
    /// Returns a copy of the pixel's alpha component.
    ///
    /// Due to a naming conflict with several now-deprecated inherent
    /// functions with the same name (such as `Rgb::alpha()`) the
    /// `HasAlpha::alpha()` method requires fully qualified syntax for
    /// disambiguation. The deprecated functions are due to be removed in a
    /// future release which will solve this issue.
    ///
    /// # Examples
    /// ```
    /// use rgb::{Rgba, HasAlpha};
    ///
    /// let mut rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// assert_eq!(HasAlpha::alpha(&rgba), 50);
    ///
    /// rgba.a = 0;
    ///
    /// assert_eq!(HasAlpha::alpha(&rgba), 0);
    /// ```
    fn alpha(&self) -> Self::AlphaComponent;
    /// Returns a mutable borrow of the pixel's alpha component.
    ///
    /// # Examples
    /// ```
    /// use rgb::{Rgba, HasAlpha};
    ///
    /// let mut rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// let alpha = rgba.alpha_mut();
    ///
    /// *alpha += 100;
    ///
    /// assert_eq!(HasAlpha::alpha(&rgba), 150);
    /// ```
    fn alpha_mut(&mut self) -> &mut Self::AlphaComponent;
}

macro_rules! has_alpha {
    ($name:ident, $alpha_bit: tt) => {
        impl<T, A> HasAlpha for $name<T, A>
        where
            T: Copy + 'static,
            A: Copy + 'static,
        {
            fn alpha(&self) -> Self::AlphaComponent {
                self.$alpha_bit
            }
            fn alpha_mut(&mut self) -> &mut Self::AlphaComponent {
                &mut self.$alpha_bit
            }
        }
    };
}

has_alpha!(Rgba, a);
has_alpha!(Argb, a);
has_alpha!(Bgra, a);
has_alpha!(Abgr, a);
has_alpha!(GrayA, a);

#[cfg(feature = "legacy")]
use crate::formats::gray_alpha::GrayAlpha_v08;
#[cfg(feature = "legacy")]
has_alpha!(GrayAlpha_v08, 1);
