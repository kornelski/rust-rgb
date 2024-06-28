#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Brightness` pixel.
///
/// This pixel is commonly used for grayscale images.
///
/// # Examples
///
/// ```
/// use rgb::Luma;
///
/// let pixel: Luma<u8> = Luma { l: 0 };
/// ```
pub struct Luma<T> {
    /// Brightness Component
    pub l: T,
}
