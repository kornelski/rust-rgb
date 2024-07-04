#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Brightness + Alpha` pixel.
///
/// This pixel is commonly used for grayscale images. A binary
/// grayscale images can also be represented using `GrayA<bool, u8>`.
///
/// # Examples
///
/// ```
/// use rgb::GrayA;
///
/// let pixel: GrayA<u8> = GrayA { v: 0, a: 255 };
/// ```
pub struct GrayA<T, A = T> {
    /// Brightness Component
    pub v: T,
    /// Alpha Component
    pub a: A,
}
