use std;

/// Casting the struct to slices of its components
pub trait ComponentSlice<T> {
    /// The components interpreted as an array, e.g. RGB gives 3-element slice. The red component is first.
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
}

/// Casting a slice of `RGB/A` values to a slice of `u8`
pub trait ComponentBytes<T: Copy> where Self: ComponentSlice<T> {
    /// The components interpreted as raw bytes, in machine's native endian. Bytes of the red component are first.
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        let slice = self.as_slice();
        unsafe {
            std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * std::mem::size_of::<T>())
        }
    }

    #[inline]
    fn as_bytes_mut(&mut self) -> &[u8] {
        let slice = self.as_mut_slice();
        unsafe {
            std::slice::from_raw_parts_mut(slice.as_ptr() as *mut u8, slice.len() * std::mem::size_of::<T>())
        }
    }
}

/// Applying operation to every component
///
/// ```rust,ignore
/// let inverted = pixel.map(|c| 255 - c);
///
/// For simple math there are Add/Sub/Mul implementations:
/// let halved = pixel.map(|c| c / 2);
/// let halved = pixel / 2;
/// ```
pub trait ComponentMap<DestPixel, SrcComponent, DestComponent> {
    /// Convenience function (equivalent of `self.iter().map().collect()`) for applying the same formula to every component.
    ///
    /// Note that it returns the pixel directly, not an Interator.
    fn map<Callback>(&self, f: Callback) -> DestPixel
        where Callback: FnMut(SrcComponent) -> DestComponent;
}
