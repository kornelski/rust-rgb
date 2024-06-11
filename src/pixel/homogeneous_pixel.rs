use crate::*;

/// A Pixel made up of a compile-time known number of contiguously stored color components and optionally an
/// alpha component.
///
/// Unlike [`HeterogeneousPixel`] the alpha component must be the same type as the color
/// components.
///
/// # Terminology
///
/// Component = An element of a pixel, inclusive of alpha. For example, [`Rgba`](crate::Rgba) is a pixel made up
/// of four components, three color components and one alpha component.
pub trait HomogeneousPixel:
    HeterogeneousPixel<ColorComponent = Self::Component, AlphaComponent = Self::Component>
{
    /// The component type of the pixel used for both color and alpha components if any.
    type Component: PixelComponent;

    /// The same pixel type as `Self` but with a different component type `U`
    type SelfType<U: PixelComponent>: HomogeneousPixel<
        Component = U,
        SelfType<Self::Component> = Self,
    >;
    //TODO if const generic expressions become stable remove this associated type and just use
    //`[Self::Component; Self::COMPONENT_COUNT]` as the return type.
    //
    /// The component array form of `Self`
    type ComponentArray<R>: IntoIterator<Item = R> + AsSlice<R>;

    /// Converts an owned `Pixel` type to an array of its components.
    fn component_array(&self) -> Self::ComponentArray<Self::Component>;

    /// Creates a new instance given an iterator of its components.
    ///
    /// # Panics
    ///
    /// This function will panic if the iterator does not produce enough components for the pixel.
    fn from_components(components: impl IntoIterator<Item = Self::Component>) -> Self;

    /// Maps each of the pixels components with a function `f` to any other component type.
    ///
    /// See [`HomogeneousPixel::map_components_same()`] if you want to map the components to the
    /// same type.
    fn map_components<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U>
    where
        U: PixelComponent;

    /// Maps each of the pixels components with a function `f` to the same component type.
    ///
    /// See [`HomogeneousPixel::map_components()`] if you want to map the components to a
    /// different type.
    fn map_components_same<U>(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self;
}

macro_rules! without_alpha {
    ($name:ident, $length:literal, [$($color_bit:tt),*]) => {
        impl<T> HomogeneousPixel for $name<T>
        where
            T: PixelComponent,
        {
            type Component = T;

            type ComponentArray<R> = [R; $length];

            fn component_array(&self) -> Self::ComponentArray<Self::Component> {
                [$(self.$color_bit),*]
            }

            fn from_components(
                components: impl IntoIterator<Item = Self::Component>,
            ) -> Self {
                let mut iter = components.into_iter();
                Self {$($color_bit: iter.next().expect("components iterator does not contain enough components for this pixel")),*}
            }

            fn map_components<U>(&self, mut f: impl FnMut(Self::Component) -> U) -> SelfType
            {
                Self {$($color_bit: f(self.$color_bit),)*}
            }
        }
    }
}

macro_rules! with_alpha {
    ($name:tt, $length:literal, [$($bit:tt),*], [$($color_bit:tt),*], $alpha_bit:tt) => {
        impl<T, A> HomogeneousPixel for $name<T, A>
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
