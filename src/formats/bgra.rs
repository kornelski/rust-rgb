#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `BGRA` pixel.
pub struct Bgra<T, A = T> {
    /// Blue Component
    pub b: T,
    /// Green Component
    pub g: T,
    /// Red Component
    pub r: T,
    /// Alpha Component
    pub a: A,
}
