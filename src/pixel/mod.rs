use crate::AsSlice;
use crate::PixelComponent;

pub mod as_slice;
pub mod contiguous_pixel;
pub mod pixel_component;

/// A Pixel made up of a compile-time known number of contiguously stored `T`s.
///
/// Pixels can optionally contain a single alpha component.
///
/// # Terminology
///
/// Component = An element of a pixel, inclusive of alpha. For example, [`Rgba`](crate::Rgba) is a pixel made up
/// of four components, three color components and one alpha component.
pub trait Pixel: Copy {
    /// The component type of the pixel used for both color and alpha components if any.
    type Component: PixelComponent;

    /// The total number of components in the pixel.
    ///
    /// If the pixel contains an alpha components then this number should be equal to the number of
    /// color components + 1. That is, you cannot have more than 1 alpha components, but you can
    /// have 0.
    const COMPONENT_COUNT: u8;

    /// The same pixel type as `Self` but with a different component type `U`
    type SelfType<U: PixelComponent>: Pixel<Component = U, SelfType<Self::Component> = Self>;
    //TODO if const generic expressions become stable remove this associated type and just use
    //`[Self::Component; Self::COMPONENT_COUNT]` as the return type.
    //
    /// The component array form of `Self`
    type ComponentArray<R>: IntoIterator<Item = R> + AsSlice<R>;
    //TODO if const generic expressions become stable remove this associated type and just use
    //`[Self::Component; Self::COMPONENT_COUNT]` as the return type.
    //
    /// The color array form of `Self`
    type ColorArray<R>: IntoIterator<Item = R> + AsSlice<R>;

    /// Converts an owned `Pixel` type to an array of its components.
    fn component_array(&self) -> Self::ComponentArray<Self::Component>;
    /// Converts an owned `Pixel` type to an array of its color components.
    fn color_array(&self) -> Self::ColorArray<Self::Component>;
    /// Returns the alpha component of the pixel if it has one.
    fn alpha(&self) -> Option<Self::Component>;

    /// Creates a new instance given an iterator of its components.
    ///
    /// # Panics
    ///
    /// This function will panic if the iterator does not produce enough components for the pixel.
    fn from_components(components: impl IntoIterator<Item = Self::Component>) -> Self;

    /// Create a new instance given an array of its color components and an alpha component.
    ///
    /// # Panics
    ///
    /// This function will panic if the colors iterator does not produce enough color components for the pixel.
    fn from_colors_alpha(
        colors: impl IntoIterator<Item = Self::Component>,
        alpha: Self::Component,
    ) -> Self;

    /// Maps each of the pixels components with a function `f`.
    fn map_components<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U>
    where
        U: PixelComponent;

    /// Maps each of the pixels color components with a function `f`.
    fn map_colors(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self;

    /// Maps the pixels alpha components with a function `f`.
    ///
    /// If the pixel has no alpha component then the pixel is returned unchanged.
    fn map_alpha(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self;
}

macro_rules! implement_pixel_without_alpha {
    ($name:ident, $length:literal, [$($bit:ident),*]) => {
        impl<T> Pixel for $name<T>
        where
            T: PixelComponent,
        {
            type Component = T;

            const COMPONENT_COUNT: u8 = $length;

            type SelfType<U: PixelComponent> = $name<U>;
            type ComponentArray<R> = [R; $length];
            type ColorArray<R> = [R; $length];

            fn component_array(&self) -> Self::ComponentArray<Self::Component> {
                [$(self.$bit),*]
            }
            fn color_array(&self) -> Self::ColorArray<Self::Component> {
                [$(self.$bit),*]
            }
            fn alpha(&self) -> Option<Self::Component> {
                None
            }

            fn from_components(components: impl IntoIterator<Item = Self::Component>) -> Self {
                let mut iter = components.into_iter();
                Self {$($bit: iter.next().expect("components iterator does not contain enough components for this pixel")),*}
            }
            fn from_colors_alpha(
                colors: impl IntoIterator<Item = Self::Component>,
                _: Self::Component,
            ) -> Self {
                let mut iter = colors.into_iter();
                Self {$($bit: iter.next().expect("colors iterator does not contain enough components for this pixel")),*}
            }

            fn map_components<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U>
            where
                U: PixelComponent,
            {
                Self::SelfType::from_components(self.component_array().map(f))
            }

            fn map_colors(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self
            {
                self.map_components(f)
            }

            fn map_alpha(&self, _: impl FnMut(Self::Component) -> Self::Component) -> Self
            {
                *self
            }
        }
    }
}

macro_rules! implement_pixel_with_alpha {
    ($name:tt, $length:literal, [$($bit:ident),*], [$($color_bit:ident),*], $alpha_bit:ident) => {
        impl<T> Pixel for $name<T>
        where
            T: PixelComponent,
        {
            type Component = T;

            const COMPONENT_COUNT: u8 = $length;

            type SelfType<U: PixelComponent> = $name<U>;
            type ComponentArray<R> = [R; $length];
            type ColorArray<R> = [R; $length - 1];

            fn component_array(&self) -> Self::ComponentArray<Self::Component> {
                [$(self.$bit),*]
            }
            fn color_array(&self) -> Self::ColorArray<Self::Component> {
                [$(self.$color_bit),*]
            }
            fn alpha(&self) -> Option<Self::Component> {
                Some(self.$alpha_bit)
            }

            fn from_components(components: impl IntoIterator<Item = Self::Component>) -> Self {
                let mut iter = components.into_iter();
                Self {$($bit: iter.next().expect("components iterator does not contain enough components for this pixel")),*}
            }
            fn from_colors_alpha(
                colors: impl IntoIterator<Item = Self::Component>,
                alpha: Self::Component,
            ) -> Self {
                let mut iter = colors.into_iter();
                Self {$($color_bit: iter.next().expect("colors iterator does not contain enough components for this pixel")),*, $alpha_bit: alpha}
            }

            fn map_components<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U>
            where
                U: PixelComponent,
            {
                Self::SelfType::from_components(self.component_array().map(f))
            }

            fn map_colors(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self
            {
                Self::SelfType::from_colors_alpha(self.color_array().map(f), self.$alpha_bit)
            }

            fn map_alpha(&self, mut f: impl FnMut(Self::Component) -> Self::Component) -> Self
            {
                Self::SelfType::from_colors_alpha(self.color_array(), f(self.$alpha_bit))
            }
        }
    }
}

mod rgba {
    use crate::*;
    implement_pixel_with_alpha!(Rgba, 4, [r, g, b, a], [r, g, b], a);
}
mod abgr {
    use crate::*;
    implement_pixel_with_alpha!(Abgr, 4, [a, b, g, r], [b, g, r], a);
}
mod argb {
    use crate::*;
    implement_pixel_with_alpha!(Argb, 4, [a, r, g, b], [r, g, b], a);
}
mod bgra {
    use crate::*;
    implement_pixel_with_alpha!(Bgra, 4, [b, g, r, a], [b, g, r], a);
}
mod gray_alpha {
    use crate::*;
    implement_pixel_with_alpha!(GrayAlpha, 2, [gray, a], [gray], a);
}

mod gray {
    use crate::*;
    implement_pixel_without_alpha!(Gray, 1, [gray]);
}
mod bgr {
    use crate::*;
    implement_pixel_without_alpha!(Bgr, 3, [b, g, r]);
}
mod rgb {
    use crate::*;
    implement_pixel_without_alpha!(Rgb, 3, [r, g, b]);
}
