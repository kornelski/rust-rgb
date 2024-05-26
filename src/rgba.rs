use crate::{Pixel, PixelExt};

#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// An `RGBA` pixel.
///
/// The component type can be `u8` (aliased as [`RgbaU8`]), `u16` (aliased as [`RgbaU16`]),
/// or any other type (but simple copyable types are recommended).
///
/// You can specify a different type for alpha, but it's only for special cases
/// (e.g. if you use a newtype like `Rgba<LinearLight<u16>, u16>`).
pub struct Rgba<T, A = T> {
    /// Red Component
    pub r: T,
    /// Green Component
    pub g: T,
    /// Blue Component
    pub b: T,
    /// Alpha Component
    pub a: A,
}

impl<T> Pixel<T, 4> for Rgba<T> {
    fn into_components(self) -> [T; 4] {
        [self.a, self.b, self.g, self.r]
    }

    fn as_components_ref(&self) -> &[T; 4] {
        unsafe { core::mem::transmute(self) }
    }

    fn as_components_mut(&mut self) -> &mut [T; 4] {
        unsafe { core::mem::transmute(self) }
    }

    fn from_components(components: [T; 4]) -> Self {
        let mut iter = components.into_iter();

        Self {
            a: iter.next().unwrap(),
            b: iter.next().unwrap(),
            g: iter.next().unwrap(),
            r: iter.next().unwrap(),
        }
    }

    fn from_components_ref(components: &[T; 4]) -> &Self {
        unsafe { core::mem::transmute(components) }
    }

    fn from_components_mut(components: &mut [T; 4]) -> &mut Self {
        unsafe { core::mem::transmute(components) }
    }
}

impl<T> PixelExt<T, 4> for Rgba<T> {
    type PixelWithComponent<U> = Rgba<U>;
}
