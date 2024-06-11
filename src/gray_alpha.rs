#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Grayscale + Alpha` pixel.
pub struct GrayA<T>(
    /// Grayscale Component
    pub T,
    /// Alpha Component
    pub T,
);
