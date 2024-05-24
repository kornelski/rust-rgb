#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `BGR` pixel.
///
/// The component type can be `u8` (aliased as [`BgrU8`]), `u16` (aliased as [`BgrU16`]),
/// or any other type (but simple copyable types are recommended).
pub struct Bgr<T> {
    /// Blue Component
    pub b: T,
    /// Green Component
    pub g: T,
    /// Red Component
    pub r: T,
}
