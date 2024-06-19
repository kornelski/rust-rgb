#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Brightness + Alpha` pixel.
///
/// This pixel is commonly used for grayscale images.
pub struct LumaA<T, A = T> {
    /// Brightness Component
    pub l: T,
    /// Alpha Component
    pub a: A,
}
