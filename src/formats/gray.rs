#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Grayscale` pixel
///
/// This is the legacy gray pixel type as opposed to the new gray type
/// (`rgb::Gray_v09`). This type is kept for backwards-compatibility.
///
/// You should transition to the new gray pixel type as this type is
/// due to be removed in a future release.
#[allow(non_camel_case_types)]
pub struct Gray_v08<T>(
    /// Grayscale Component. This field will be renamed to `v`.
    pub T,
);

/// A `Grayscale` pixel.
///
/// This is the new gray pixel type as opposed to the legacy gray type
/// (`rgb::Gray`) which is kept for backwards-compatibility.
///
/// # Examples
///
/// ```
/// use rgb::Gray_v09 as Gray;
///
/// let pixel: Gray<u8> = Gray { v: 0 };
/// ```
#[allow(non_camel_case_types)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[doc(alias = "Luma")]
#[doc(alias = "Mono")]
#[doc(alias = "GRAY8")]
#[doc(alias = "GRAY16")]
pub struct Gray_v09<T> {
    /// Grayscale Component
    pub v: T,
}

impl<T> core::ops::Deref for Gray_v08<T> {
    type Target = Gray_v09<T>;

    fn deref(&self) -> &Gray_v09<T> {
        unsafe {
            &*(self as *const Self as *const Gray_v09::<T>)
        }
    }
}

#[test]
fn swizzle() {
    let g = Gray_v08(10u8);
    assert_eq!(10, g.v);
    assert_eq!(10, g.0);
}
