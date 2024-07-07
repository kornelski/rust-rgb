#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Grayscale` pixel (rgb crate v0.8)
#[allow(non_camel_case_types)]
pub struct Gray_v08<T>(
    /// Grayscale Component. This field will be renamed to `v`.
    pub T,
);

/// A `Grayscale` pixel (rgb crate v0.9)
///
/// # Examples
///
/// ```
/// use rgb::Gray;
///
/// let pixel: Gray<u8> = Gray { v: 0 };
/// ```
#[allow(non_camel_case_types)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[doc(alias = "Luma")]
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
