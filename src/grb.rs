#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `GRB` pixel.
///
/// The component type can be `u8` (aliased as [`GrbU8`]), `u16` (aliased as [`GrbU16`]),
/// or any other type (but simple copyable types are recommended).
pub struct Grb<T> {
    /// Green Component
    pub g: T,
    /// Red Component
    pub r: T,
    /// Blue Component
    pub b: T,
}
