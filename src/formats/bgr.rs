#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Blue + Green + Red` pixel.
pub struct Bgr<T> {
    /// Blue Component
    pub b: T,
    /// Green Component
    pub g: T,
    /// Red Component
    pub r: T,
}
