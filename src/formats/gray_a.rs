#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Value (brightness) + Alpha` pixel (rgb crate v0.9)
///
/// This pixel is commonly used for grayscale images.
pub struct GrayA<T, A = T> {
    /// Value - the brightness component. May be luma or luminance.
    pub v: T,
    /// Alpha Component
    pub a: A,
}
