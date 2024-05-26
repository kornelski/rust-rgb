/// A Pixel made up of a compile-time known number of contiguously stored `T`s.
///
/// Usually `T` is a small copiable intrinsic type such as `u8`, `u16` or `f32`.
pub trait Pixel<T, const N: usize> {
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
}

/// Pixel extension trait for helper functions
pub trait PixelExt<T, const N: usize>: Pixel<T, N> {
    /// The same pixel type as Self but with a different generic component type.
    type PixelWithComponent<U>;

    /// Map a Pixel<T> to a different Pixel<U>
    fn map<U>(&self, f: impl FnMut(T) -> U) -> Self::PixelWithComponent<U>
    where
        Self::PixelWithComponent<U>: Pixel<U, N>,
        Self: Copy,
    {
        Self::PixelWithComponent::from_components(self.into_components().map(f))
    }
}
