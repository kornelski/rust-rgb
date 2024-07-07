/// Casting the struct to slices of its components
pub trait ComponentSlice<T> {
    /// The components interpreted as an array, e.g. one `RGB` expands to 3 elements.
    ///
    /// It's implemented for individual pixels as well as slices of pixels.
    fn as_slice(&self) -> &[T];

    /// The components interpreted as a mutable array, e.g. one `RGB` expands to 3 elements.
    ///
    /// It's implemented for individual pixels as well as slices of pixels.
    ///
    /// If you get an error when calling this on an array, add `[..]`
    ///
    /// > use of unstable library feature 'array_methods'
    ///
    /// ```rust,ignore
    /// arr[..].as_mut_slice()
    /// ```
    fn as_mut_slice(&mut self) -> &mut [T];
}

/// Casting a slice of `RGB/A` values to a slice of `u8`
///
/// If instead of `RGB8` you use `RGB<MyCustomType>`, and you want to cast from/to that custom type,
/// implement the `Plain` trait for it:
///
/// ```rust
/// # #[derive(Copy, Clone)]
/// # struct MyCustomType;
/// unsafe impl rgb::Pod for MyCustomType {}
/// unsafe impl rgb::Zeroable for MyCustomType {}
/// ```
///
/// Plain types are not allowed to contain struct padding, booleans, chars, enums, references or pointers.
#[cfg(feature = "as-bytes")]
pub trait ComponentBytes<T: ::bytemuck::Pod> {
    /// The components interpreted as raw bytes, in machine's native endian. In `RGB` bytes of the red component are first.
    fn as_bytes(&self) -> &[u8];

    /// The components interpreted as raw bytes, in machine's native endian. In `RGB` bytes of the red component are first.
    fn as_bytes_mut(&mut self) -> &mut [u8];
}

impl<T: ::bytemuck::Pod> ComponentBytes<T> for [T] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
       assert_ne!(0, core::mem::size_of::<T>());
       ::bytemuck::cast_slice(self)
    }

    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        ::bytemuck::cast_slice_mut(self)
    }
}

/// Same as `Pixel::map`, but doesn't change the alpha channel (if there's any alpha).
pub trait ColorComponentMap<DestPixel, SrcComponent, DestComponent> {
    /// Convenience function for applying the same formula to every rgb/gray component, but skipping the alpha component.
    ///
    /// Note that it returns the pixel directly, not an Interator.
    fn map_c<Callback>(&self, f: Callback) -> DestPixel
        where Callback: FnMut(SrcComponent) -> DestComponent;
}
