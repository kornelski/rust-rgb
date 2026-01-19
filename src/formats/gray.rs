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
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Copy> Gray_v08<T> {
    /// Reads the `.0` field
    ///
    /// This is a compatibility shim. Migrate to `Gray_v09` and use `.v` directly.
    #[deprecated(since = "0.8.91", note = "Use Gray_v09 with .v field instead")]
    pub fn value(self) -> T {
        self.0
    }

    /// Exposes the `.0` field for writing
    ///
    /// This is a compatibility shim. Migrate to `Gray_v09` and use `.v` directly.
    #[deprecated(since = "0.8.91", note = "Use Gray_v09 with .v field instead")]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Add alpha component to this pixel
    ///
    /// This is a compatibility shim. Migrate to `Gray_v09` and use `GrayA` instead.
    #[deprecated(since = "0.8.91", note = "Use Gray_v09::with_alpha() or GrayA instead")]
    #[allow(deprecated)]
    pub fn with_alpha(self, add_alpha_value: T) -> crate::formats::gray_alpha::GrayAlpha_v08<T> {
        crate::formats::gray_alpha::GrayAlpha_v08(self.0, add_alpha_value)
    }
}
