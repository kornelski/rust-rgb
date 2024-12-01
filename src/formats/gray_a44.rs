use core::fmt::Display;

#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Grayscale + Alpha` 8-bit pixel with the first 4 bits used for the value
/// component the last 4 bits used for the alpha component.
///
/// # Examples
///
/// ```
/// use rgb::GrayA44;
///
/// let pixel = GrayA44::new(0, 15).unwrap();
/// ```
pub struct GrayA44(pub(crate) u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Error returned from the [`GrayA44::new()`] function if the given
/// components are out of range.
pub struct OutOfRangeError;
impl Display for OutOfRangeError {
    #[cold]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("the components are outside of the valid range")
    }
}

impl GrayA44 {
    /// Tries to create new [`GrayA44`] given both components.
    ///
    /// Returns an error if either component is out of the 4-bit range (`0..=15`).
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::GrayA44;
    /// use rgb::error::OutOfRangeError;
    ///
    /// let pixel = GrayA44::new(0, 15).unwrap();
    /// assert_eq!(pixel.v(), 0);
    /// assert_eq!(pixel.a(), 15);
    ///
    /// assert_eq!(GrayA44::new(120, 20), Err(OutOfRangeError));
    /// ```
    #[inline]
    pub fn new(v: u8, a: u8) -> Result<Self, OutOfRangeError> {
        if v > 15 || a > 15 {
            return Err(OutOfRangeError);
        }

        Ok(Self((v << 4) | a))
    }

    /// Returns a copy of the pixel's 4-bit grayscale component as a `u8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::GrayA44;
    ///
    /// let pixel = GrayA44::new(0, 15).unwrap();
    ///
    /// assert_eq!(pixel.v(), 0);
    /// ```
    #[inline]
    pub fn v(self) -> u8 {
        self.0 >> 4
    }

    #[inline]
    /// Returns a copy of the pixel's 4-bit alpha component as a `u8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgb::GrayA44;
    ///
    /// let pixel = GrayA44::new(0, 15).unwrap();
    ///
    /// assert_eq!(pixel.a(), 15);
    /// ```
    pub fn a(self) -> u8 {
        self.0 & 0xF
    }
}
