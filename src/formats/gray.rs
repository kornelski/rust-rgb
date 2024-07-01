#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Brightness` pixel.
///
/// This pixel is commonly used for grayscale images. A binary
/// grayscale images can also be represented using `Gray<bool>`.
///
/// # Examples
///
/// ```
/// use rgb::Gray;
///
/// let pixel: Gray<u8> = Gray { v: 0 };
/// ```
pub struct Gray<T> {
    /// Brightness Component
    pub v: T,
}
