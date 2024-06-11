#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// An `RGB` pixel.
pub struct Rgb<T> {
    /// Red Component
    pub r: T,
    /// Green Component
    pub g: T,
    /// Blue Component
    pub b: T,
}
