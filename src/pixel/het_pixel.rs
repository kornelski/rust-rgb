use core::fmt::Display;
use crate::PixelComponent;
use crate::{Abgr, Argb, ArrayLike, Bgr, Bgra, Gray, GrayA, Grb, Rgb, Rgba, Rgbw};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
/// Unlike [`HomPixel`](crate::HomPixel) the alpha component does not have to be the same type as the color
/// components.
///
/// This trait is implemented on every pixel type in the crate.
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
    ///
    /// For example, [`Rgb`] has a `COMPONENT_COUNT` == 3 whereas
    /// [`Rgba`] has a `COMPONENT_COUNT` == 4.
    const COMPONENT_COUNT: u8;

    /// The same pixel type as `Self` but with a different component type `U`.
    ///
    /// This is used to allow the implementation of
    /// [`HetPixel::map_colors`] and similar methods due to rust's
    /// current lack of higher kinded types.
    ///
    /// For example, [`Rgb`] has `SelfType<U, V> = Rgb<U>` whereas
    /// [`Rgba`] has `SelfType<U, V> = Rgba<U, V>`.
    type SelfType<U: PixelComponent, V: PixelComponent>: HetPixel<
        SelfType<Self::ColorComponent, Self::AlphaComponent> = Self,
    >;

    /// An generic associated type used to return the array of color
    /// components despite rust's lack of const generic expressions.
    ///
    /// Used in functions like [`HetPixel::color_array()`].
    ///
    /// For example, [`Rgb`] has `ColorArray<U> = [U; 3]` and
    /// [`Rgba`] has `ColorArray<U> = [U; 3]` also.
    type ColorArray<U>: ArrayLike<U>;

    /// Returns an owned array of copies of the pixels color components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{HetPixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// assert_eq!(rgb.color_array(), [0, 10, 100]);
    /// assert_eq!(rgba.color_array(), [0, 10, 100]);
    /// ```
    //TODO switch to returning an plain array if const generic expressions ever stabilize
    fn color_array(&self) -> Self::ColorArray<Self::ColorComponent>
    where
        Self::ColorArray<Self::ColorComponent>: Copy;
    /// Returns an owned array of the pixel's mutably borrowed color components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{HetPixel, Rgb, Rgba};
    ///
    /// let mut rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let mut rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// *rgb.color_array_mut()[1] = 40;
    /// *rgba.color_array_mut()[2] = 40;
    ///
    /// assert_eq!(rgb.color_array(), [0, 40, 100]);
    /// assert_eq!(rgba.color_array(), [0, 10, 40]);
    /// ```
    //TODO switch to returning an plain array if const generic expressions ever stabilize
    fn color_array_mut(&mut self) -> Self::ColorArray<&mut Self::ColorComponent>;

    /// Returns a copy of the pixel's alpha alpha component if it has one.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{HetPixel, Rgb, Rgba};
    ///
    /// let mut rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let mut rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// assert_eq!(rgb.alpha_checked(), None);
    /// assert_eq!(rgba.alpha_checked(), Some(50));
    /// ```
    fn alpha_checked(&self) -> Option<Self::AlphaComponent>;
    /// Returns a mutable borrow of the pixel's alpha alpha component if it has one.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{HetPixel, Rgb, Rgba};
    ///
    /// let mut rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let mut rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// let f = |a: Option<&mut u8>| {
    ///     if let Some(a) = a {
    ///         *a -= 10;
    ///     }
    /// };
    ///
    /// f(rgb.alpha_checked_mut());
    /// f(rgba.alpha_checked_mut());
    ///
    /// assert_eq!(rgb.alpha_checked(), None);
    /// assert_eq!(rgba.alpha_checked(), Some(40));
    /// ```
    fn alpha_checked_mut(&mut self) -> Option<&mut Self::AlphaComponent>;

    /// Tries to create new instance given an iterator of color components and an alpha component.
    ///
    /// Returns an error if the `colors` iterator does not contain enough items to create the pixel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{HetPixel, Rgb, Rgba, TryFromColorsAlphaError};
    ///
    /// let mut values2 = [0_u8, 10];
    /// let mut values4 = [0_u8, 10, 100, 40];
    ///
    /// let alpha = 50;
    ///
    /// assert_eq!(Rgb::try_from_colors_alpha(values2, alpha), Err(TryFromColorsAlphaError));
    /// assert_eq!(Rgba::try_from_colors_alpha(values2, alpha), Err(TryFromColorsAlphaError));
    ///
    /// assert_eq!(Rgb::try_from_colors_alpha(values4, alpha), Ok(Rgb {r: 0, g: 10, b: 100}));
    /// assert_eq!(Rgba::try_from_colors_alpha(values4, alpha), Ok(Rgba {r: 0, g: 10, b: 100, a: 50}));
    /// ```
    fn try_from_colors_alpha(
        colors: impl IntoIterator<Item = Self::ColorComponent>,
        alpha: Self::AlphaComponent,
    ) -> Result<Self, TryFromColorsAlphaError>;

    /// Maps each of the pixels color components with a function `f` to any other type.
    ///
    /// See [`HetPixel::map_colors_same()`] if you want to map the color components to the
    /// same type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{HetPixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// let f = |color: u8| {
    ///     u16::from(color) * 10
    /// };
    ///
    /// assert_eq!(rgb.map_colors(f), Rgb {r: 0, g: 100, b: 1000});
    /// assert_eq!(rgba.map_colors(f), Rgba {r: 0, g: 100, b: 1000, a: 50});
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{HetPixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// let f = |color: u8| {
    ///     color / 2
    /// };
    ///
    /// assert_eq!(rgb.map_colors_same(f), Rgb {r: 0, g: 5, b: 50});
    /// assert_eq!(rgba.map_colors_same(f), Rgba {r: 0, g: 5, b: 50, a: 50});
    /// ```
    fn map_colors_same(&self, f: impl FnMut(Self::ColorComponent) -> Self::ColorComponent) -> Self;

    /// Maps the pixels alpha component with a function `f` to any other type.
    ///
    /// If the pixel has no alpha component then the pixel is returned unchanged.
    ///
    /// See [`HetPixel::map_alpha_same()`] if you want to map the alpha component to the
    /// same type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{HetPixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// let f = |alpha: u8| {
    ///     u16::from(alpha) * 10
    /// };
    ///
    /// assert_eq!(rgb.map_alpha(f), Rgb {r: 0, g: 10, b: 100});
    /// assert_eq!(rgba.map_alpha(f), Rgba {r: 0, g: 10, b: 100, a: 500});
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{HetPixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// let f = |alpha: u8| {
    ///     alpha / 2
    /// };
    ///
    /// assert_eq!(rgb.map_alpha_same(f), Rgb {r: 0, g: 10, b: 100});
    /// assert_eq!(rgba.map_alpha_same(f), Rgba {r: 0, g: 10, b: 100, a: 25});
    /// ```
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
			type ColorArray<U> = [U; $length];

			fn color_array(&self) -> Self::ColorArray<Self::ColorComponent>
			where
				Self::ColorArray<Self::ColorComponent>: Copy
			{
                [$(self.$color_bit),*]
            }
			fn color_array_mut(&mut self) -> Self::ColorArray<&mut Self::ColorComponent>
			{
                [$(&mut self.$color_bit),*]
            }

            fn alpha_checked(&self) -> Option<Self::AlphaComponent> {
                None
            }
            fn alpha_checked_mut(&mut self) -> Option<&mut Self::AlphaComponent> {
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
			type ColorArray<U> = [U; $length - 1];

			fn color_array(&self) -> Self::ColorArray<Self::ColorComponent>
			where
				Self::ColorArray<Self::ColorComponent>: Copy
			{
                [$(self.$color_bit),*]
            }
			fn color_array_mut(&mut self) -> Self::ColorArray<&mut Self::ColorComponent>
			{
                [$(&mut self.$color_bit),*]
            }

            fn alpha_checked(&self) -> Option<Self::AlphaComponent> {
                Some(self.$alpha_bit)
            }
            fn alpha_checked_mut(&mut self) -> Option<&mut Self::AlphaComponent> {
                Some(&mut self.$alpha_bit)
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
with_alpha!(GrayA, 2, [v], a);

without_alpha!(Bgr, 3, [b, g, r]);
without_alpha!(Rgb, 3, [r, g, b]);
without_alpha!(Grb, 3, [r, g, b]);
without_alpha!(Gray, 1, [v]);
without_alpha!(Rgbw, 4, [r, g, b, w]);
