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

impl<T: Copy> Gray_v09<T> {
    /// Value - the brightness component. May be luma or luminance.
    ///
    /// Backwards-compatible getter for `self.v`. Do not use unless necessary.
    #[doc(hidden)]
    pub fn value(&self) -> T {
        self.v
    }
}

impl<T: Copy> Gray_v08<T> {
    /// Value - the brightness component. May be luma or luminance.
    ///
    /// Backwards-compatible getter for the missing `self.v`.
    #[doc(alias = "v")]
    pub fn value(&self) -> T {
        self.0
    }
}

// Rust can't deprecate trait impls
/// DO NOT USE IT. Not compatible with 0.9
impl<T> core::ops::Deref for Gray_v08<T> {
    type Target = T;

    /// DO NOT USE IT. Not compatible with 0.9
    fn deref(&self) -> &T {
        &self.0
    }
}
