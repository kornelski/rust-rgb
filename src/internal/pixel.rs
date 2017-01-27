use std;

/// Casting the struct to slices/bytes of its components
pub trait ComponentBytes<T> {
    /// The components interpreted as an array, e.g. RGB gives 3-element slice. The red component is first.
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];

    /// The components interpreted as raw bytes, in machine's native endian. Bytes of the red component are first.
    fn as_bytes(&self) -> &[u8] {
        let slice = self.as_slice();
        unsafe {
            std::slice::from_raw_parts(std::mem::transmute(slice.as_ptr()), slice.len() * std::mem::size_of::<T>())
        }
    }
}

/// Applying operation to every component
pub trait ComponentMap<DestPixel, SrcComponent, DestComponent> {
    /// Convenience function (equivalent of `self.iter().map().collect()`) for applying same formula to every component
    fn map<Callback>(&self, f: Callback) -> DestPixel
        where Callback: FnMut(SrcComponent) -> DestComponent;
}
