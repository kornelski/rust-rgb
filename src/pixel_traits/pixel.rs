use core::fmt::Display;
use crate::HetPixel;
use crate::{Abgr, Argb, ArrayLike, Bgr, Bgra, Gray, GrayA, Grb,Rgb, Rgba, Rgbw};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Error returned from the [`Pixel::try_from_components()`] function.
pub struct TryFromComponentsError;
impl Display for TryFromComponentsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the components iterator did not contain enough items to create this pixel"
        )
    }
}

/// A Pixel made up of a compile-time known number of color components and optionally an
/// alpha component.
///
/// Unlike [`HetPixel`] the alpha component must be the same type as the color
/// components.
///
/// This trait is implemented on every pixel type in the crate.
///
/// All types which implement [`Pixel`] also implement [`HetPixel`] due to the super-trait trait bound.
pub trait Pixel:
    HetPixel<ColorComponent = Self::Component, AlphaComponent = Self::Component>
    + IntoIterator<Item = Self::Component>
{
    /// The component type of the pixel used for both color and alpha components if any.
    type Component: Copy + 'static;

    /// An generic associated type used to return the array of
    /// components despite rust's lack of const generic expressions.
    ///
    /// Used in functions like [`Pixel::component_array()`].
    ///
    /// For example, [`Rgb`] has `ComponentArray<U> = [U; 3]` wheareas
    /// [`Rgba`] has `ComponentArray<U> = [U; 4]`.
    type ComponentArray<U>: ArrayLike<U>;

    /// Returns an owned array of copies of the pixels components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// assert_eq!(rgb.component_array(), [0, 10, 100]);
    /// assert_eq!(rgba.component_array(), [0, 10, 100, 50]);
    /// ```
    //TODO switch to returning an plain array if const generic expressions ever stabilize
    fn component_array(&self) -> Self::ComponentArray<Self::Component>
    where
        Self::ComponentArray<Self::Component>: Copy;
    /// Returns an owned array of the pixel's mutably borrowed components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let mut rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let mut rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// *rgb.component_array_mut()[1] = 40;
    /// *rgba.component_array_mut()[2] = 40;
    ///
    /// assert_eq!(rgb.component_array(), [0, 40, 100]);
    /// assert_eq!(rgba.component_array(), [0, 10, 40, 50]);
    /// ```
    //TODO switch to returning an plain array if const generic expressions ever stabilize
    fn component_array_mut(&mut self) -> Self::ComponentArray<&mut Self::Component>;

    /// Tries to create new instance given an iterator of its components.
    ///
    /// Returns an error if the `components` iterator does not contain enough items to create the pixel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba, TryFromComponentsError};
    ///
    /// let mut values2 = [0_u8, 10];
    /// let mut values4 = [0_u8, 10, 100, 40];
    ///
    /// assert_eq!(Rgb::try_from_components(values2), Err(TryFromComponentsError));
    /// assert_eq!(Rgba::try_from_components(values2), Err(TryFromComponentsError));
    ///
    /// assert_eq!(Rgb::try_from_components(values4), Ok(Rgb {r: 0, g: 10, b: 100}));
    /// assert_eq!(Rgba::try_from_components(values4), Ok(Rgba {r: 0, g: 10, b: 100, a: 40}));
    /// ```
    fn try_from_components(
        components: impl IntoIterator<Item = Self::Component>,
    ) -> Result<Self, TryFromComponentsError>;

    /// Maps each of the pixels components with a function `f` to any other component type.
    ///
    /// See [`Pixel::map_components_same()`] if you want to map the components to the
    /// same type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// let f = |color: u8| {
    ///     u16::from(color) * 10
    /// };
    ///
    /// assert_eq!(rgb.map_components(f), Rgb {r: 0, g: 100, b: 1000});
    /// assert_eq!(rgba.map_components(f), Rgba {r: 0, g: 100, b: 1000, a: 500});
    /// ```
    fn map_components<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U, U>
    where
        U: Copy;
    /// Maps each of the pixels components with a function `f` to the same component type.
    ///
    /// See [`Pixel::map_components()`] if you want to map the components to a
    /// different type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// let f = |color: u8| {
    ///     color / 2
    /// };
    ///
    /// assert_eq!(rgb.map_components_same(f), Rgb {r: 0, g: 5, b: 50});
    /// assert_eq!(rgba.map_components_same(f), Rgba {r: 0, g: 5, b: 50, a: 25});
    /// ```
    fn map_components_same(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self;
}

macro_rules! without_alpha {
    ($name:tt, $length:literal, [$($bit:tt),*]) => {
        impl<T> Pixel for $name<T>
        where
            T: Copy + 'static,
        {
            type Component = T;
			type ComponentArray<U> = [U; $length];

			fn component_array(&self) -> Self::ComponentArray<Self::Component>
			where
				Self::ComponentArray<Self::Component>: Copy
			{
                [$(self.$bit),*]
            }
			fn component_array_mut(&mut self) -> Self::ComponentArray<&mut Self::Component> {
                [$(&mut self.$bit),*]
            }

            fn try_from_components(
                components: impl IntoIterator<Item = Self::Component>,
            ) -> Result<Self, TryFromComponentsError> {
                let mut iter = components.into_iter();
                Ok(Self {$($bit: iter.next().ok_or(TryFromComponentsError)?),*})
            }

            fn map_components<U>(&self, mut f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U, U>
                where U: Copy + 'static
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
        impl<T> Pixel for $name<T, T>
        where
            T: Copy + 'static,
        {
            type Component = T;
			type ComponentArray<U> = [U; $length];

			fn component_array(&self) -> Self::ComponentArray<Self::Component>
			where
				Self::ComponentArray<Self::Component>: Copy
			{
                [$(self.$bit),*]
            }
			fn component_array_mut(&mut self) -> Self::ComponentArray<&mut Self::Component> {
                [$(&mut self.$bit),*]
            }

            fn try_from_components(
                components: impl IntoIterator<Item = Self::Component>,
            ) -> Result<Self, TryFromComponentsError> {
                let mut iter = components.into_iter();
                Ok(Self {$($bit: iter.next().ok_or(TryFromComponentsError)?),*})
            }

            fn map_components<U>(&self, mut f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U, U>
                where U: Copy + 'static
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
with_alpha!(GrayA, 2, [v, a]);

without_alpha!(Bgr, 3, [b, g, r]);
without_alpha!(Rgb, 3, [r, g, b]);
without_alpha!(Grb, 3, [r, g, b]);
without_alpha!(Gray, 1, [v]);
without_alpha!(Rgbw, 4, [r, g, b, w]);
