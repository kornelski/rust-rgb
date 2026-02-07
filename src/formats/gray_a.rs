#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Grayscale + Alpha` pixel (rgb crate v0.9)
///
/// This pixel is commonly used for grayscale images. A binary
/// grayscale images can also be represented using `GrayA<u8, bool>`,
/// but this won't reduce the storage size.
///
/// This is the new gray+alpha pixel type as opposed to the legacy gray+alpha type
/// (`rgb::GrayAlpha`) which is kept for backwards-compatibility.
///
/// # Examples
///
/// ```
/// use rgb::GrayA;
///
/// let pixel: GrayA<u8> = GrayA { v: 0, a: 255 };
/// ```
#[doc(alias = "GrayAlpha")]
#[doc(alias = "GRAYA8")]
#[doc(alias = "GRAYA16")]
pub struct GrayA<T, A = T> {
    /// Value - the brightness component. May be luma or luminance.
    pub v: T,
    /// Alpha Component
    pub a: A,
}

impl<T: Copy, A> GrayA<T, A> {
    /// Value - the brightness component. May be luma or luminance.
    ///
    /// Prefer `.v` whenever possible. Backwards-compatible getter for `self.v`
    pub fn value(&self) -> T {
        self.v
    }
}
