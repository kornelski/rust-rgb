use crate::formats::gray_a::GrayA;
use core::ops::Deref;
use core::ops::DerefMut;

#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A pixel for grayscale value + alpha components (rgb crate v0.8)
///
/// This is the legacy gray+alpha pixel type as opposed to the new gray+alpha type
/// (`rgb::GrayA`). This type is kept for backwards-compatibility.
///
/// You should transition to the new gray+alpha pixel type as this type is
/// due to be removed in a future release.
///
/// Through a `Deref` hack it renames the fields from `.0` and `.1`
/// to `.v` (value) and `.a` (alpha)
#[allow(non_camel_case_types)]
pub struct GrayAlpha_v08<T, A = T>(
    /// Grayscale Component
    ///
    /// This field has been renamed to `.v`
    pub T,
    /// Alpha Component. This field has been renamed to `.a`.
    pub A,
);

impl<T, A> Deref for GrayAlpha_v08<T, A> {
    type Target = GrayA<T, A>;

    /// A trick that allows using `.v` and `.a` on the old `GrayAlpha` type.
    fn deref(&self) -> &GrayA<T, A> {
        unsafe { &*core::ptr::from_ref(self).cast::<GrayA<T, A>>() }
    }
}

impl<T, A> DerefMut for GrayAlpha_v08<T, A> {
    /// A trick that allows using `.v` and `.a` on the old `GrayAlpha` type.
    fn deref_mut(&mut self) -> &mut GrayA<T, A> {
        unsafe { &mut *core::ptr::from_mut(self).cast::<GrayA<T, A>>() }
    }
}

impl<T: Clone, A> GrayAlpha_v08<T, A> {
    /// Value - the brightness component. May be luma or luminance.
    ///
    /// Backwards-compatible getter for `self.v`
    pub fn value(&self) -> T {
        self.0.clone()
    }
}

#[test]
fn swizzle() {
    let g = GrayAlpha_v08(10_u8, 20_u8);
    assert_eq!(10, g.v);
    assert_eq!(20, g.a);
}
