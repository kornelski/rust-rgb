#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Red + Green + Blue + White` pixel.
///
/// # Examples
///
/// ```
/// use rgb::Rgbw;
///
/// let pixel: Rgbw<u8> = Rgbw { r: 0, g: 0, b: 0, w: 0 };
/// ```
pub struct Rgbw<T> {
    /// Red Component
    pub r: T,
    /// Green Component
    pub g: T,
    /// Blue Component
    pub b: T,
    /// White Component
    pub w: T,
}
