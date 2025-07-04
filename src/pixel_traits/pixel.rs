use core::fmt::Display;
use crate::HetPixel;
use crate::{Abgr, Argb, ArrayLike, Bgr, Bgra, Gray_v09, GrayA, Grb,Rgb, Rgba, Rgbw};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Error returned from the [`Pixel::try_from_components()`] function.
pub struct TryFromComponentsError;
impl Display for TryFromComponentsError {
    #[cold]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("the components iterator did not contain enough items to create this pixel")
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
{
    /// The component type of the pixel used for both color and alpha components if any.
    type Component: Copy;

    /// A generic associated type used to return the array of
    /// components despite rust's lack of const generic expressions.
    ///
    /// Used in functions like [`Pixel::to_array()`].
    ///
    /// For example, [`Rgb`] has `ComponentArray<U> = [U; 3]` wheareas
    /// [`Rgba`] has `ComponentArray<U> = [U; 4]`.
    type ComponentArray<U>: ArrayLike<U>;

    /// Returns an owned array of copies of the pixel's components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb { r: 0_u8, g: 10, b: 100 };
    /// let rgba = Rgba { r: 0_u8, g: 10, b: 100, a: 50 };
    ///
    /// assert_eq!(rgb.to_array(), [0, 10, 100]);
    /// assert_eq!(rgba.to_array(), [0, 10, 100, 50]);
    /// ```
    //TODO switch to returning an plain array if const generic expressions ever stabilize
    #[doc(alias = "into_array")]
    #[doc(alias = "component_array")]
    fn to_array(&self) -> Self::ComponentArray<Self::Component>
    where Self::ComponentArray<Self::Component>: Copy;

    /// Casts a reference of the pixel to an array reference of the pixel's
    /// components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// assert_eq!(rgb.as_array(), &[0, 10, 100]);
    /// assert_eq!(rgba.as_array(), &[0, 10, 100, 50]);
    /// ```
    #[doc(alias = "as_ref")]
    fn as_array(&self) -> &Self::ComponentArray<Self::Component>;

    /// Casts a mutable reference of the pixel to an mutable array reference of the pixel's
    /// components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let mut rgb = Rgb {r: 0_u8, g: 10, b: 100};
    /// let mut rgba = Rgba {r: 0_u8, g: 10, b: 100, a: 50};
    ///
    /// rgb.as_array_mut()[1] = 40;
    /// rgba.as_array_mut()[2] = 40;
    ///
    /// assert_eq!(rgb.as_array(), &[0, 40, 100]);
    /// assert_eq!(rgba.as_array(), &[0, 10, 40, 50]);
    /// ```
    #[doc(alias = "as_mut")]
    fn as_array_mut(&mut self) -> &mut Self::ComponentArray<Self::Component>;

    /// Returns an owned array of the pixel's mutably borrowed components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let mut rgb = Rgb { r: 0_u8, g: 10, b: 100 };
    /// let mut rgba = Rgba { r: 0_u8, g: 10, b: 100, a: 50 };
    ///
    /// *rgb.each_mut()[1] = 40;
    /// *rgba.each_mut()[2] = 40;
    ///
    /// assert_eq!(rgb.to_array(), [0, 40, 100]);
    /// assert_eq!(rgba.to_array(), [0, 10, 40, 50]);
    /// ```
    //TODO switch to returning an plain array if const generic expressions ever stabilize
    #[doc(alias = "to_array_mut")]
    fn each_mut(&mut self) -> Self::ComponentArray<&mut Self::Component>;

    /// Tries to create new instance given an iterator of its components.
    ///
    /// Returns an error if the `components` iterator does not contain enough items to create the pixel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::error::TryFromComponentsError;
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let mut values2 = [0_u8, 10];
    /// let mut values4 = [0_u8, 10, 100, 40];
    ///
    /// assert_eq!(Rgb::try_from_components(values2), Err(TryFromComponentsError));
    /// assert_eq!(Rgba::try_from_components(values2), Err(TryFromComponentsError));
    ///
    /// assert_eq!(Rgb::try_from_components(values4), Ok(Rgb { r: 0, g: 10, b: 100 }));
    /// assert_eq!(Rgba::try_from_components(values4), Ok(Rgba { r: 0, g: 10, b: 100, a: 40 }));
    /// ```
    fn try_from_components(
        components: impl IntoIterator<Item = Self::Component>,
    ) -> Result<Self, TryFromComponentsError>;

    /// Maps each of the pixel's components with a function `f` to any other component type.
    ///
    /// See [`Pixel::map_same()`] if you want to map the components to the
    /// same type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb { r: 0_u8, g: 10, b: 100 };
    /// let rgba = Rgba { r: 0_u8, g: 10, b: 100, a: 50 };
    ///
    /// let widen = |b: u8| u16::from(b) << 8 | u16::from(b);
    ///
    /// assert_eq!(rgb.map(widen), Rgb { r: 0, g: 2570, b: 25700 });
    /// assert_eq!(rgba.map(widen), Rgba { r: 0, g: 2570, b: 25700, a: 50*256+50 });
    /// ```
    fn map<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U, U> where U: Copy;

    /// Maps each of the pixel's components with a function `f` to the same component type.
    ///
    /// Use [`Pixel::map()`] if you want to map the components to a
    /// different type. `map()` can also be used to keep the same component type,
    /// but due to limitations of Rust's type system,`map_same()` may be required in generic contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::{Pixel, Rgb, Rgba};
    ///
    /// let rgb = Rgb { r: 0_u8, g: 10, b: 100 };
    /// let rgba = Rgba { r: 0_u8, g: 10, b: 100, a: 50 };
    ///
    /// let halved = |color: u8| color / 2;
    ///
    /// assert_eq!(rgb.map_same(halved), Rgb { r: 0, g: 5, b: 50 });
    /// assert_eq!(rgba.map_same(halved), Rgba { r: 0, g: 5, b: 50, a: 25 });
    /// ```
    fn map_same(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self;
}

macro_rules! without_alpha {
    ($name:tt, $length:literal, [$($bit:tt),*]) => {
        impl<T> Pixel for $name<T> where T: Copy + 'static {
            type Component = T;
            type ComponentArray<U> = [U; $length];

            #[inline]
            fn to_array(&self) -> Self::ComponentArray<Self::Component> where Self::ComponentArray<Self::Component>: Copy {
                [$(self.$bit),*]
            }

            #[inline]
            fn as_array(&self) -> &Self::ComponentArray<Self::Component> {
                unsafe {
                    &*(self as *const Self).cast()
                }
            }

            #[inline]
            fn as_array_mut(&mut self) -> &mut Self::ComponentArray<Self::Component> {
                unsafe {
                    &mut *(self as *mut Self).cast()
                }
            }

            #[inline]
            fn each_mut(&mut self) -> Self::ComponentArray<&mut Self::Component> {
                [$(&mut self.$bit),*]
            }

            #[inline]
            fn try_from_components(components: impl IntoIterator<Item = Self::Component>) -> Result<Self, TryFromComponentsError> {
                let mut iter = components.into_iter();
                Ok(Self {$($bit: iter.next().ok_or(TryFromComponentsError)?),*})
            }

            #[inline]
            fn map<U>(&self, mut f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U, U> where U: Copy + 'static {
                $name { $($bit: f(self.$bit),)* }
            }

            #[inline]
            fn map_same(&self, mut f: impl FnMut(Self::Component) -> Self::Component) -> Self {
                $name { $($bit: f(self.$bit),)* }
            }
        }
    }
}

macro_rules! with_alpha {
    ($name:tt, $length:literal, [$($bit:tt),*]) => {
        impl<T> Pixel for $name<T, T> where T: Copy + 'static {
            type Component = T;
            type ComponentArray<U> = [U; $length];

            #[inline]
            fn to_array(&self) -> Self::ComponentArray<Self::Component> where Self::ComponentArray<Self::Component>: Copy {
                [$(self.$bit),*]
            }

            #[inline]
            fn as_array(&self) -> &Self::ComponentArray<Self::Component> {
                unsafe {
                    &*(self as *const Self).cast()
                }
            }

            #[inline]
            fn as_array_mut(&mut self) -> &mut Self::ComponentArray<Self::Component> {
                unsafe {
                    &mut *(self as *mut Self).cast()
                }
            }

            #[inline]
            fn each_mut(&mut self) -> Self::ComponentArray<&mut Self::Component> {
                [$(&mut self.$bit),*]
            }

            #[inline]
            fn try_from_components(components: impl IntoIterator<Item = Self::Component>) -> Result<Self, TryFromComponentsError> {
                let mut iter = components.into_iter();
                Ok(Self {$($bit: iter.next().ok_or(TryFromComponentsError)?),*})
            }

            #[inline]
            fn map<U>(&self, mut f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U, U> where U: Copy + 'static {
                $name { $($bit: f(self.$bit),)* }
            }

            #[inline]
            fn map_same(&self, mut f: impl FnMut(Self::Component) -> Self::Component) -> Self {
                $name { $($bit: f(self.$bit),)* }
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
without_alpha!(Gray_v09, 1, [v]);
without_alpha!(Rgbw, 4, [r, g, b, w]);

use crate::formats::gray::Gray_v08;
without_alpha!(Gray_v08, 1, [0]);

use crate::formats::gray_alpha::GrayAlpha_v08;
with_alpha!(GrayAlpha_v08, 2, [0, 1]);


#[test]
fn as_refs() {
    let mut r = Rgba::new(1_u8, 2, 3, 4u8);
    assert_eq!(&[1, 2, 3, 4], r.as_array());
    assert_eq!(&[1, 2, 3, 4], AsRef::<[u8; 4]>::as_ref(&r));
    assert_eq!(&[1, 2, 3, 4], r.as_ref());
    assert_eq!([1, 2, 3, 4], *r.as_array_mut());
    assert_eq!([1, 2, 3, 4], *AsMut::<[u8; 4]>::as_mut(&mut r));
    assert_eq!([1, 2, 3, 4], *r.as_mut());

    let mut r = GrayA::new(1_u8, 4u8);
    assert_eq!(&[1, 4], r.as_array());
    assert_eq!(&[1, 4], AsRef::<[u8; 2]>::as_ref(&r));
    assert_eq!(&[1, 4], r.as_ref());
    assert_eq!([1, 4], *r.as_array_mut());
    assert_eq!([1, 4], *AsMut::<[u8; 2]>::as_mut(&mut r));
    assert_eq!([1, 4], *r.as_mut());
}
