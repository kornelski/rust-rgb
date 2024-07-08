use crate::{RGB, RGBA};
use crate::alt::{Gray, GrayAlpha, BGR, BGRA};
use crate::alt::{ARGB, ABGR};
use crate::ComponentBytes;

#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Pod for RGB<T> where T: crate::Pod {}
#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Pod for BGR<T> where T: crate::Pod {}
#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Zeroable for RGB<T> where T: crate::Zeroable {}
#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Zeroable for BGR<T> where T: crate::Zeroable {}

#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Zeroable for ABGR<T, A> where T: crate::Zeroable, A: crate::Zeroable {
    #[track_caller]
    #[inline(always)]
    fn zeroed() -> Self {
        unsafe {
            let _ = assert_no_padding::<T, A, Self>();
            core::mem::zeroed()
        }
    }
}

#[track_caller]
const fn assert_no_padding<T, A, S>() {
    if core::mem::size_of::<A>() + 3 * core::mem::size_of::<T>() != core::mem::size_of::<S>() {
        panic!("type has padding");
    }
}

#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Pod for RGBA<T, A> where T: crate::Pod, A: crate::Pod {}

#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Pod for BGRA<T, A> where T: crate::Pod, A: crate::Pod {}

#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Zeroable for RGBA<T, A> where T: crate::Zeroable, A: crate::Zeroable {
    #[track_caller]
    #[inline(always)]
    fn zeroed() -> Self {
        unsafe {
            let _ = assert_no_padding::<T, A, Self>();
            core::mem::zeroed()
        }
    }
}

#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Pod for ARGB<T, A> where T: crate::Pod, A: crate::Pod {}

#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Pod for ABGR<T, A> where T: crate::Pod, A: crate::Pod {}

#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Zeroable for ARGB<T, A> where T: crate::Zeroable, A: crate::Zeroable {
    #[track_caller]
    #[inline(always)]
    fn zeroed() -> Self {
        unsafe {
            let _ = assert_no_padding::<T, A, Self>();
            core::mem::zeroed()
        }
    }
}

#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Zeroable for BGRA<T, A> where T: crate::Zeroable, A: crate::Zeroable {
    #[track_caller]
    #[inline(always)]
    fn zeroed() -> Self {
        unsafe {
            let _ = assert_no_padding::<T, A, Self>();
            core::mem::zeroed()
        }
    }
}

#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Pod for Gray<T> where T: crate::Pod {}
#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Pod for GrayAlpha<T, A> where T: crate::Pod, A: crate::Pod {}
#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Zeroable for Gray<T> where T: crate::Zeroable {}
#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Zeroable for GrayAlpha<T, A> where T: crate::Zeroable, A: crate::Zeroable {
    #[track_caller]
    #[inline(always)]
    fn zeroed() -> Self {
        unsafe {
            if core::mem::size_of::<A>() + core::mem::size_of::<T>() != core::mem::size_of::<Self>() {
                panic!("type has padding");
            }
            core::mem::zeroed()
        }
    }
}

#[cfg(feature = "as-bytes")]
impl<T: crate::Pod> ComponentBytes<T> for [Gray<T>] {}

#[cfg(feature = "as-bytes")]
impl<T: crate::Pod> ComponentBytes<T> for [GrayAlpha<T>] {}
