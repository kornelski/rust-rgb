#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Grayscale + Alpha` pixel.
///
/// The component type can be `u8` (aliased as [`GrayAlphaU8`]), `u16` (aliased as [`GrayAlphaU16`]),
/// or any other type (but simple copyable types are recommended).
///
/// You can specify a different type for alpha, but it's only for special cases
/// (e.g. if you use a newtype like `GrayAlpha<LinearLight<u16>, u16>`).
pub struct GrayAlpha<T, A = T> {
    /// Grayscale Component
    pub gray: T,
    /// Alpha Component
    pub a: A,
}
