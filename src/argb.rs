#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// An `ARGB` pixel.
///
/// The component type can be `u8` (aliased as [`ArgbU8`]), `u16` (aliased as [`ArgbU16`]),
/// or any other type (but simple copyable types are recommended).
///
/// You can specify a different type for alpha, but it's only for special cases
/// (e.g. if you use a newtype like `Argb<LinearLight<u16>, u16>`).
pub struct Argb<T, A = T> {
    /// Alpha Component
    pub a: A,
    /// Red Component
    pub r: T,
    /// Green Component
    pub g: T,
    /// Blue Component
    pub b: T,
}
