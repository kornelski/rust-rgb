/// A Pixel made up of a compile-time known number of contiguously stored `T`s.
///
/// Usually `T` is a small copiable intrinsic type such as `u8`, `u16` or `f32`.
pub trait Pixel<T, const N: usize> {
    /// Converts an owned `Pixel` type to an array of its components.
    #[allow(clippy::wrong_self_convention)]
    fn as_components(self) -> [T; N];
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
