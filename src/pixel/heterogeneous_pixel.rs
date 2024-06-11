use crate::*;

/// A Pixel made up of a compile-time known number of contiguously stored color components and optionally an
/// alpha component.
///
/// Unlike [`HomogeneousPixel`] the alpha component does not have to be the same type as the color
/// components.
///
/// # Terminology
///
/// Component = An element of a pixel, inclusive of alpha. For example, [`Rgba`](crate::Rgba) is a pixel made up
/// of four components, three color components and one alpha component.
pub trait HeterogeneousPixel: Copy {
    /// The component type of the pixel used the color component(s).
    type ColorComponent: PixelComponent;
    /// The component type of the pixel used the alpha component if any.
    type AlphaComponent: PixelComponent;

    /// The total number of components in the pixel.
    ///
    /// If the pixel contains an alpha components then this number should be equal to the number of
    /// color components + 1. That is, you cannot have more than 1 alpha components, but you can
    /// have 0.
    const COMPONENT_COUNT: u8;

    //TODO if const generic expressions become stable remove this associated type and just use
    //`[Self::Component; Self::COMPONENT_COUNT]` as the return type.
    //
    /// The color array form of `Self`
    type ColorArray<R>: IntoIterator<Item = R> + AsSlice<R>;

    /// Converts an owned `Pixel` type to an array of its color components.
    fn color_array(&self) -> Self::ColorArray<Self::ColorComponent>;
    /// Returns the alpha component of the pixel if it has one.
    fn alpha(&self) -> Option<Self::AlphaComponent>;

    /// Create a new instance given an array of its color components and an alpha component.
    ///
    /// # Panics
    ///
    /// This function will panic if the colors iterator does not produce enough color components for the pixel.
    fn from_colors_alpha(
        colors: impl IntoIterator<Item = Self::ColorComponent>,
        alpha: Self::AlphaComponent,
    ) -> Self;

    /// Maps each of the pixels color components with a function `f`.
    fn map_colors(&self, f: impl FnMut(Self::ColorComponent) -> Self::ColorComponent) -> Self;

    /// Maps the pixels alpha components with a function `f`.
    ///
    /// If the pixel has no alpha component then the pixel is returned unchanged.
    fn map_alpha(&self, f: impl FnMut(Self::AlphaComponent) -> Self::AlphaComponent) -> Self;
}

macro_rules! without_alpha {
    ($name:ident, $length:literal, [$($color_bit:tt),*]) => {
        impl<T> HeterogeneousPixel for $name<T>
        where
            T: PixelComponent,
        {
            type ColorComponent = T;
            type AlphaComponent = T;

            const COMPONENT_COUNT: u8 = $length;

            type ColorArray<R> = [R; $length];

            fn color_array(&self) -> Self::ColorArray<Self::ColorComponent> {
                [$(self.$color_bit),*]
            }
            fn alpha(&self) -> Option<Self::AlphaComponent> {
                None
            }

            fn from_colors_alpha(
                colors: impl IntoIterator<Item = Self::ColorComponent>,
                _: Self::AlphaComponent,
            ) -> Self {
                let mut iter = colors.into_iter();
                Self {$($color_bit: iter.next().expect("colors iterator does not contain enough components for this pixel")),*}
            }

            fn map_colors(&self, mut f: impl FnMut(Self::ColorComponent) -> Self::ColorComponent) -> Self
            {
                Self {$($color_bit: f(self.$color_bit),)*}
            }

            fn map_alpha(&self, _: impl FnMut(Self::AlphaComponent) -> Self::AlphaComponent) -> Self
            {
                *self
            }
        }
    }
}

macro_rules! with_alpha {
    ($name:tt, $length:literal, [$($bit:tt),*], [$($color_bit:tt),*], $alpha_bit:tt) => {
        impl<T, A> HeterogeneousPixel for $name<T, A>
        where
            T: PixelComponent,
            A: PixelComponent,
        {
            type ColorComponent = T;
            type AlphaComponent = A;

            const COMPONENT_COUNT: u8 = $length;

            type ColorArray<R> = [R; $length - 1];

            fn color_array(&self) -> Self::ColorArray<Self::ColorComponent> {
                [$(self.$color_bit),*]
            }
            fn alpha(&self) -> Option<Self::AlphaComponent> {
                Some(self.$alpha_bit)
            }

            fn from_colors_alpha(
                colors: impl IntoIterator<Item = Self::ColorComponent>,
                alpha: Self::AlphaComponent,
            ) -> Self {
                let mut iter = colors.into_iter();
                Self {$($color_bit: iter.next().expect("colors iterator does not contain enough components for this pixel")),*, $alpha_bit: alpha}
            }

            fn map_colors(&self, f: impl FnMut(Self::ColorComponent) -> Self::ColorComponent) -> Self
            {
                Self::from_colors_alpha(self.color_array().map(f), self.$alpha_bit)
            }

            fn map_alpha(&self, mut f: impl FnMut(Self::AlphaComponent) -> Self::AlphaComponent) -> Self
            {
                Self::from_colors_alpha(self.color_array(), f(self.$alpha_bit))
            }
        }
    }
}

mod rgba {
    use crate::*;
    with_alpha!(Rgba, 4, [r, g, b, a], [r, g, b], a);
}
mod abgr {
    use crate::*;
    with_alpha!(Abgr, 4, [a, b, g, r], [b, g, r], a);
}
mod argb {
    use crate::*;
    with_alpha!(Argb, 4, [a, r, g, b], [r, g, b], a);
}
mod bgra {
    use crate::*;
    with_alpha!(Bgra, 4, [b, g, r, a], [b, g, r], a);
}
mod gray_alpha {
    use crate::*;
    with_alpha!(GrayA, 2, [0, 1], [0], 1);
}

mod gray {
    use crate::*;
    without_alpha!(Gray, 1, [0]);
}
mod bgr {
    use crate::*;
    without_alpha!(Bgr, 3, [b, g, r]);
}
mod rgb {
    use crate::*;
    without_alpha!(Rgb, 3, [r, g, b]);
}
