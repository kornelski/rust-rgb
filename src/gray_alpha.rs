#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Grayscale + Alpha` pixel.
pub struct GrayAlpha<T> {
    /// Grayscale Component
    pub gray: T,
    /// Alpha Component
    pub a: T,
}
