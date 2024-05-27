#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Grayscale` pixel.
///
/// The component type can be `u8` (aliased as [`GrayU8`]), `u16` (aliased as [`GrayU16`]),
/// or any other type (but simple copyable types are recommended).
pub struct Gray<T> {
    /// Grayscale Component
    pub gray: T,
}
