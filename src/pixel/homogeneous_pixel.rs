use core::fmt::Display;

use crate::*;

#[derive(Debug, Clone, Copy)]
/// Error returned from the [`HomogeneousPixel::try_from_components()`] function.
pub struct TryFromComponentsError;
impl Display for TryFromComponentsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the components iterator did not contain enough items to create this pixel"
        )
    }
}

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

    //TODO switch to returning an plain array if const generic expressions ever stabilize
    /// Converts an owned `Pixel` type to an array of its components.
    fn component_array(&self) -> impl ArrayLike<Self::Component>;

    /// Tries to create new instance given an iterator of its components.
    ///
    /// Returns an error if the `components` iterator does not contain enough items to create the pixel.
    fn try_from_components(
        components: impl IntoIterator<Item = Self::Component>,
    ) -> Result<Self, TryFromComponentsError>;

    /// Maps each of the pixels components with a function `f` to any other component type.
    ///
    /// See [`HomogeneousPixel::map_components_same()`] if you want to map the components to the
    /// same type.
    fn map_components<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U, U>
    where
        U: PixelComponent;
    /// Maps each of the pixels components with a function `f` to the same component type.
    ///
    /// See [`HomogeneousPixel::map_components()`] if you want to map the components to a
    /// different type.
    fn map_components_same(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self;
}

macro_rules! without_alpha {
    ($name:tt, $length:literal, [$($bit:tt),*]) => {
        impl<T> HomogeneousPixel for $name<T>
        where
            T: PixelComponent,
        {
            type Component = T;

            fn component_array(&self) -> impl ArrayLike<Self::Component> {
                [$(self.$bit),*]
            }

            fn try_from_components(
                components: impl IntoIterator<Item = Self::Component>,
            ) -> Result<Self, TryFromComponentsError> {
                let mut iter = components.into_iter();
                Ok(Self {$($bit: iter.next().ok_or(TryFromComponentsError)?),*})
            }

            fn map_components<U>(&self, mut f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U, U>
                where U: PixelComponent
            {
                $name {$($bit: f(self.$bit),)*}
            }
            fn map_components_same(&self, mut f: impl FnMut(Self::Component) -> Self::Component) -> Self
            {
                $name {$($bit: f(self.$bit),)*}
            }
        }
    }
}
macro_rules! with_alpha {
    ($name:tt, $length:literal, [$($bit:tt),*]) => {
        impl<T> HomogeneousPixel for $name<T, T>
        where
            T: PixelComponent,
        {
            type Component = T;

            fn component_array(&self) -> impl ArrayLike<Self::Component> {
                [$(self.$bit),*]
            }

            fn try_from_components(
                components: impl IntoIterator<Item = Self::Component>,
            ) -> Result<Self, TryFromComponentsError> {
                let mut iter = components.into_iter();
                Ok(Self {$($bit: iter.next().ok_or(TryFromComponentsError)?),*})
            }

            fn map_components<U>(&self, mut f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U, U>
                where U: PixelComponent
            {
                $name {$($bit: f(self.$bit),)*}
            }
            fn map_components_same(&self, mut f: impl FnMut(Self::Component) -> Self::Component) -> Self
            {
                $name {$($bit: f(self.$bit),)*}
            }
        }
    }
}

with_alpha!(Rgba, 4, [r, g, b, a]);
with_alpha!(Abgr, 4, [a, b, g, r]);
with_alpha!(Argb, 4, [a, r, g, b]);
with_alpha!(Bgra, 4, [b, g, r, a]);
with_alpha!(GrayA, 2, [0, 1]);

without_alpha!(Bgr, 3, [b, g, r]);
without_alpha!(Rgb, 3, [r, g, b]);
without_alpha!(Grb, 3, [r, g, b]);
without_alpha!(Gray, 1, [0]);
