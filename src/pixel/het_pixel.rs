use core::fmt::Display;

use crate::*;

#[derive(Debug, Clone, Copy)]
/// Error returned from the [`HetPixel::try_from_colors_alpha()`] function.
pub struct TryFromColorsAlphaError;
impl Display for TryFromColorsAlphaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the colors iterator did not contain enough items to create this pixel"
        )
    }
}

/// A Pixel made up of a compile-time known number of color components and optionally an
/// alpha component.
///
/// Unlike [`HomPixel`] the alpha component does not have to be the same type as the color
/// components.
///
/// # Terminology
///
/// Component = An element of a pixel, inclusive of alpha. For example, [`Rgba`](crate::Rgba) is a pixel made up
/// of four components, three color components and one alpha component.
pub trait HetPixel: Copy + 'static {
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

    /// The same pixel type as `Self` but with a different component type `U`
    type SelfType<U: PixelComponent, V: PixelComponent>: HetPixel<
        SelfType<Self::ColorComponent, Self::AlphaComponent> = Self,
    >;

    //TODO switch to returning an plain array if const generic expressions ever stabilize
    /// Converts an owned `Pixel` type to an array of its color components.
    fn color_array(&self) -> impl ArrayLike<Self::ColorComponent>;
    /// Returns the alpha component of the pixel if it has one.
    fn alpha_checked(&self) -> Option<Self::AlphaComponent>;

    /// Tries to create new instance given an iterator of color components and an alpha component.
    ///
    /// Returns an error if the `colors` iterator does not contain enough items to create the pixel.
    fn try_from_colors_alpha(
        colors: impl IntoIterator<Item = Self::ColorComponent>,
        alpha: Self::AlphaComponent,
    ) -> Result<Self, TryFromColorsAlphaError>;

    /// Maps each of the pixels color components with a function `f` to any other type.
    ///
    /// See [`HetPixel::map_colors_same()`] if you want to map the color components to the
    /// same type.
    fn map_colors<U>(
        &self,
        f: impl FnMut(Self::ColorComponent) -> U,
    ) -> Self::SelfType<U, Self::AlphaComponent>
    where
        U: PixelComponent;
    /// Maps each of the pixels color components with a function `f` to the same type.
    ///
    /// See [`HetPixel::map_colors()`] if you want to map the color components to a
    /// different type.
    fn map_colors_same(&self, f: impl FnMut(Self::ColorComponent) -> Self::ColorComponent) -> Self;

    /// Maps the pixels alpha component with a function `f` to any other type.
    ///
    /// If the pixel has no alpha component then the pixel is returned unchanged.
    ///
    /// See [`HetPixel::map_alpha_same()`] if you want to map the alpha component to the
    /// same type.
    fn map_alpha<U>(
        &self,
        f: impl FnMut(Self::AlphaComponent) -> U,
    ) -> Self::SelfType<Self::ColorComponent, U>
    where
        U: PixelComponent;
    /// Maps the pixels alpha component with a function `f` to the same type.
    ///
    /// If the pixel has no alpha component then the pixel is returned unchanged.
    ///
    /// See [`HetPixel::map_alpha()`] if you want to map the alpha component to a
    /// different type.
    fn map_alpha_same(&self, f: impl FnMut(Self::AlphaComponent) -> Self::AlphaComponent) -> Self;
}

macro_rules! without_alpha {
    ($name:ident, $length:literal, [$($color_bit:tt),*]) => {
        impl<T> HetPixel for $name<T>
        where
            T: PixelComponent,
        {
            type ColorComponent = T;
            type AlphaComponent = T;

            const COMPONENT_COUNT: u8 = $length;

            type SelfType<U: PixelComponent, V: PixelComponent> = $name<U>;

            fn color_array(&self) -> impl ArrayLike<Self::ColorComponent> {
                [$(self.$color_bit),*]
            }
            fn alpha_checked(&self) -> Option<Self::AlphaComponent> {
                None
            }

            fn try_from_colors_alpha(
                colors: impl IntoIterator<Item = Self::ColorComponent>,
                _: Self::AlphaComponent,
            ) -> Result<Self, TryFromColorsAlphaError> {
                let mut iter = colors.into_iter();
                Ok(Self {$($color_bit: iter.next().ok_or(TryFromColorsAlphaError)?),*})
            }

            fn map_colors<U>(
                &self,
                mut f: impl FnMut(Self::ColorComponent) -> U,
            ) -> Self::SelfType<U, Self::AlphaComponent>
            where
                U: PixelComponent
            {
                $name {$($color_bit: f(self.$color_bit),)*}
            }
            fn map_colors_same(&self, mut f: impl FnMut(Self::ColorComponent) -> Self::ColorComponent) -> Self
            {
                Self {$($color_bit: f(self.$color_bit),)*}
            }

            fn map_alpha<U>(
                &self,
                _: impl FnMut(Self::AlphaComponent) -> U,
            ) -> Self::SelfType<Self::ColorComponent, U>
            where
                U: PixelComponent
            {
                *self
            }
            fn map_alpha_same(&self, _: impl FnMut(Self::AlphaComponent) -> Self::AlphaComponent) -> Self
            {
                *self
            }
        }
    }
}
macro_rules! with_alpha {
    ($name:tt, $length:literal, [$($color_bit:tt),*], $alpha_bit:tt) => {
        impl<T, A> HetPixel for $name<T, A>
        where
            T: PixelComponent,
            A: PixelComponent,
        {
            type ColorComponent = T;
            type AlphaComponent = A;

            const COMPONENT_COUNT: u8 = $length;

            type SelfType<U: PixelComponent, V: PixelComponent> = $name<U, V>;

            fn color_array(&self) -> impl ArrayLike<Self::ColorComponent> {
                [$(self.$color_bit),*]
            }
            fn alpha_checked(&self) -> Option<Self::AlphaComponent> {
                Some(self.$alpha_bit)
            }

            fn try_from_colors_alpha(
                colors: impl IntoIterator<Item = Self::ColorComponent>,
                alpha: Self::AlphaComponent,
            ) -> Result<Self, TryFromColorsAlphaError> {
                let mut iter = colors.into_iter();
                Ok(Self {$($color_bit: iter.next().ok_or(TryFromColorsAlphaError)?),*, $alpha_bit: alpha})
            }

            fn map_colors<U>(
                &self,
                mut f: impl FnMut(Self::ColorComponent) -> U,
            ) -> Self::SelfType<U, Self::AlphaComponent>
            where
                U: PixelComponent
            {
                $name {$($color_bit: f(self.$color_bit),)* $alpha_bit: self.$alpha_bit}
            }
            fn map_colors_same(&self, mut f: impl FnMut(Self::ColorComponent) -> Self::ColorComponent) -> Self
            {
                Self {$($color_bit: f(self.$color_bit),)* $alpha_bit: self.$alpha_bit}
            }

            fn map_alpha<U>(
                &self,
                mut f: impl FnMut(Self::AlphaComponent) -> U,
            ) -> Self::SelfType<Self::ColorComponent, U>
            where
                U: PixelComponent
            {
                $name {$($color_bit: self.$color_bit,)* $alpha_bit: f(self.$alpha_bit)}
            }
            fn map_alpha_same(&self, mut f: impl FnMut(Self::AlphaComponent) -> Self::AlphaComponent) -> Self
            {
                $name {$($color_bit: self.$color_bit,)* $alpha_bit: f(self.$alpha_bit)}
            }
        }
    }
}

with_alpha!(Rgba, 4, [r, g, b], a);
with_alpha!(Abgr, 4, [b, g, r], a);
with_alpha!(Argb, 4, [r, g, b], a);
with_alpha!(Bgra, 4, [b, g, r], a);
with_alpha!(GrayA, 2, [0], 1);

without_alpha!(Bgr, 3, [b, g, r]);
without_alpha!(Rgb, 3, [r, g, b]);
without_alpha!(Grb, 3, [r, g, b]);
without_alpha!(Gray, 1, [0]);
