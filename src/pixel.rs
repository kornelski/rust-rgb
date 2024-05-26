/// A Pixel made up of a compile-time known number of contiguously stored `T`s.
///
/// Usually `T` is a small copiable intrinsic type such as `u8`, `u16` or `f32`.
pub trait HomogeneousPixel<T, const N: usize> {
    /// The same pixel type as Self but with a different generic component type.
    type PixelWithComponent<U>;

    /// Converts an owned `Pixel` type to an array of its components.
    fn into_components(self) -> [T; N];
    /// Converts a reference of a `Pixel` type to a reference of an array of its components.
    fn as_components_ref(&self) -> &[T; N];
    /// Converts a mutable reference of a `Pixel` type to a mutable reference of an array of its components.
    fn as_components_mut(&mut self) -> &mut [T; N];

    /// Converts an array of components to a `Pixel`.
    fn from_components(components: [T; N]) -> Self;
    /// Converts a reference of an array of components to a refrecce of a `Pixel`.
    fn from_components_ref(components: &[T; N]) -> &Self;
    /// Converts a mutable reference of an array of components to a mutable reference of a `Pixel`.
    fn from_components_mut(components: &mut [T; N]) -> &mut Self;

    /// Map the pixel to the same outer pixel type with an optionally different inner type.
    fn map<U>(&self, f: impl FnMut(T) -> U) -> Self::PixelWithComponent<U>
    where
        Self::PixelWithComponent<U>: HomogeneousPixel<U, N>,
        Self: Copy,
    {
        Self::PixelWithComponent::from_components(self.into_components().map(f))
    }
}

/// A pixel with possibly differently typed color and alpha components.
pub trait HeterogeneousPixel<T, A, const N: usize> {
    /// The same pixel type as Self but with a different generic component types.
    type PixelWithComponent<U, B>;

    /// The color components
    fn colors(&self) -> [T; N];
    /// The alpha component
    fn alpha(&self) -> A;

    /// Create a new instance given the color and alpha components.
    fn from_colors_alpha(colors: [T; N], alpha: A) -> Self;

    /// Map the pixel to the same outer pixel type with an optionally different inner color component
    /// type and the exact same alpha component.
    fn map_colors<U>(&self, f: impl FnMut(T) -> U) -> Self::PixelWithComponent<U, A>
    where
        Self::PixelWithComponent<U, A>: HeterogeneousPixel<U, A, N>,
    {
        Self::PixelWithComponent::from_colors_alpha(self.colors().map(f), self.alpha())
    }

    /// Map the pixel to the same outer pixel type with an optionally different inner color component
    /// type and the exact same alpha component.
    fn map_alpha<B>(&self, mut f: impl FnMut(A) -> B) -> Self::PixelWithComponent<T, B>
    where
        Self::PixelWithComponent<T, B>: HeterogeneousPixel<T, B, N>,
    {
        Self::PixelWithComponent::from_colors_alpha(self.colors(), f(self.alpha()))
    }
}
