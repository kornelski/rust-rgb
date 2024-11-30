#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A pixel for grayscale value (4 bit) + alpha components (4 bit). in total 8 bit
pub struct GrayAlpha44(pub(crate) u8);

impl GrayAlpha44 {
    /// Convert 16 bit gray+alpha into 8 bit gray+alpha with precision loss
    /// Example:
    /// ```
    /// use rgb::GrayAlpha44;
    /// let g = GrayAlpha44::new(120, 20);
    /// assert_eq!(112, g.v()); // due to loss of precision you got 112 instead of initial 120
    /// assert_eq!(16, g.a()); // due to loss of precision you got 16 instead of initial 20
    /// ```
    #[inline]
    pub fn new(v: u8, a: u8) -> Self {
        Self((v & 0xf0) | (a >> 4))
    }

    #[inline]
    pub fn v(self) -> u8 {
        self.0 & 0xf0
    }

    #[inline]
    pub fn a(self) -> u8 {
        self.0 << 4
    }
}

#[test]
fn zero() {
    let g = GrayAlpha44::new(0, 0);
    assert_eq!(0, g.v());
    assert_eq!(0, g.a());
    assert_eq!(0, g.0);
}
