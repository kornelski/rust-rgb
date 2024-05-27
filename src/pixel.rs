/// A Pixel made up of a compile-time known number of contiguously stored `T`s.
///
/// Usually `T` is a small copiable intrinsic type such as `u8`, `u16` or `f32`.
pub trait HomogeneousPixel<T> {
    /// The same pixel type as Self but with a different generic component type.
    type SelfType<U>;
    /// The array form of Self
    type ArrayForm<R>;

    /// Converts an owned `Pixel` type to an array of its components.
    fn components(&self) -> Self::ArrayForm<T>;
    /// Converts an array of components to a `Pixel`.
    fn from_components(components: Self::ArrayForm<T>) -> Self;

    /// Map the pixel to the same outer pixel type with an optionally different inner type.
    fn map<U>(&self, f: impl FnMut(T) -> U) -> Self::SelfType<U>
    where
        U: Copy;
}

/// A pixel with possibly differently typed color and alpha components.
pub trait HeterogeneousPixel<T, A> {
    /// The same pixel type as Self but with a different generic component types.
    type SelfType<U, B>;
    /// The color array form of Self
    type ColorArrayForm<R>;

    /// The color components
    fn colors(&self) -> Self::ColorArrayForm<T>;
    /// The alpha component
    fn alpha(&self) -> A;

    /// Create a new instance given the color and alpha components.
    fn from_colors_alpha(colors: Self::ColorArrayForm<T>, alpha: A) -> Self;

    /// Map the pixel to the same outer pixel type with an optionally different inner color component
    /// type and the exact same alpha component.
    fn map_colors<U>(&self, f: impl FnMut(T) -> U) -> Self::SelfType<U, A>
    where
        U: Copy;

    /// Map the pixel to the same outer pixel type with an optionally different inner color component
    /// type and the exact same alpha component.
    fn map_alpha<B>(&self, f: impl FnMut(A) -> B) -> Self::SelfType<T, B>
    where
        B: Copy;
}
