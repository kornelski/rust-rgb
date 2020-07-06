
/// Casting the struct to slices of its components
pub trait ComponentSlice<T> {
    /// The components interpreted as an array, e.g. one `RGB` expands to 3 elements.
    ///
    /// It's implemented for individual pixels as well as slices of pixels.
    fn as_slice(&self) -> &[T];

    /// The components interpreted as a mutable array, e.g. one `RGB` expands to 3 elements.
    ///
    /// It's implemented for individual pixels as well as slices of pixels.
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
pub trait ComponentBytes<T: crate::Pod> where Self: ComponentSlice<T> {
    /// The components interpreted as raw bytes, in machine's native endian. In `RGB` bytes of the red component are first.
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        let slice = self.as_slice();
        unsafe {
            core::slice::from_raw_parts(slice.as_ptr() as *const _, slice.len() * core::mem::size_of::<T>())
        }
    }

    /// The components interpreted as raw bytes, in machine's native endian. In `RGB` bytes of the red component are first.
    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        let slice = self.as_mut_slice();
        unsafe {
            core::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut _, slice.len() * core::mem::size_of::<T>())
        }
    }
}

/// Applying operation to every component
///
/// ```rust
/// use rgb::ComponentMap;
/// # let pixel = rgb::RGB::new(0u8,0,0);
/// let inverted = pixel.map(|c| 255 - c);
///
/// // For simple math there are Add/Sub/Mul implementations:
/// let halved = pixel.map(|c| c / 2);
/// let doubled = pixel * 2;
/// ```
pub trait ComponentMap<DestPixel, SrcComponent, DestComponent> {
    /// Convenience function (equivalent of `self.iter().map().collect()`) for applying the same formula to every component.
    ///
    /// Note that it returns the pixel directly, not an Interator.
    fn map<Callback>(&self, f: Callback) -> DestPixel
        where Callback: FnMut(SrcComponent) -> DestComponent;
}
