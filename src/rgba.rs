use crate::{HeterogeneousPixel, HomogeneousPixel};

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

impl<T> HomogeneousPixel<T> for Rgba<T>
where
    T: Copy,
{
    type SelfType<U> = Rgba<U>;
    type ArrayForm<R> = [R; 4];

    fn components(&self) -> [T; 4] {
        [self.a, self.b, self.g, self.r]
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

    fn map<U>(&self, f: impl FnMut(T) -> U) -> Self::SelfType<U>
    where
        U: Copy,
    {
        Self::SelfType::from_components(self.components().map(f))
    }
}

impl<T, A> HeterogeneousPixel<T, A> for Rgba<T, A>
where
    T: Copy,
    A: Copy,
{
    type SelfType<U, B> = Rgba<U, B>;
    type ColorArrayForm<R> = [R; 3];

    fn colors(&self) -> [T; 3] {
        [self.r, self.g, self.b]
    }

    fn alpha(&self) -> A {
        self.a
    }

    fn from_colors_alpha(colors: [T; 3], alpha: A) -> Self {
        Self {
            r: colors[0],
            g: colors[1],
            b: colors[2],
            a: alpha,
        }
    }

    fn map_colors<U>(&self, f: impl FnMut(T) -> U) -> Self::SelfType<U, A>
    where
        U: Copy,
    {
        Self::SelfType::from_colors_alpha(self.colors().map(f), self.alpha())
    }
    fn map_alpha<B>(&self, mut f: impl FnMut(A) -> B) -> Self::SelfType<T, B>
    where
        B: Copy,
    {
        Self::SelfType::from_colors_alpha(self.colors(), f(self.alpha()))
    }
}
