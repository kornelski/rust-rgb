use crate::*;

/// A pixel which can gain an alpha component.
pub trait WithAlpha: Pixel {
    /// The pixel type with its alpha component.
    type WithAlpha: Pixel;

    /// Returns the pixel type with its alpha component. If no alpha component is already contained
    /// then it is set to the maximum value.
    /// [`PixelComponent`].
    fn with_alpha(self) -> Self::WithAlpha;
}
/// A pixel which can lose its alpha component.
pub trait WithoutAlpha: Pixel {
    /// The pixel type without its alpha component.
    type WithoutAlpha: Pixel;

    /// Returns the pixel type without its alpha component.
    fn without_alpha(self) -> Self::WithoutAlpha;
}

macro_rules! implement_lower_upper {
    ($lower:ident, $upper:ident, {$($bit:ident),*}) => {
        impl<T> WithAlpha for $lower<T> where T: PixelComponent {
            type WithAlpha = $upper<T>;

            fn with_alpha(self) -> Self::WithAlpha {
                $upper {
                    $($bit: self.$bit),*,
                    a: <$lower<T> as Pixel>::Component::COMPONENT_MAX,
                }
            }
        }
        impl<T> WithoutAlpha for $upper<T> where T: PixelComponent {
            type WithoutAlpha = $lower<T>;

            fn without_alpha(self) -> Self::WithoutAlpha {
                $lower {
                    $($bit: self.$bit),*,
                }
            }
        }
    };
}
macro_rules! implement_with_no_op {
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
macro_rules! implement_without_no_op {
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

implement_without_no_op!(Rgb);
implement_without_no_op!(Bgr);
implement_without_no_op!(Gray);

implement_with_no_op!(Rgba);
implement_with_no_op!(Argb);
implement_with_no_op!(Bgra);
implement_with_no_op!(Abgr);
implement_with_no_op!(GrayAlpha);

implement_lower_upper!(Rgb, Rgba, {r, g, b});
implement_lower_upper!(Bgr, Bgra, {r, g, b});
implement_lower_upper!(Gray, GrayAlpha, { gray });
