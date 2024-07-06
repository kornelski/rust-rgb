use crate::alt::*;
use crate::RGB;
use crate::RGBA;
use super::pixel::*;

impl<T> RGBA<T> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self {r,g,b,a}
    }
}

impl<T, A> RGBA<T,A> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    pub const fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
        Self {r,g,b,a}
    }
}

impl<T> BGRA<T> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// Warning: The order of arguments is R,G,B,A
    #[deprecated(note="This function has a misleading order of arguments. Use BGRA{} literal instead")]
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { b, g, r, a }
    }
}

impl<T, A> BGRA<T,A> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// Warning: The order of arguments is R,G,B,A
    #[deprecated(note="This function has a misleading order of arguments. Use BGRA{} literal instead")]
    pub const fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
        Self { b, g, r, a }
    }
}

#[cfg(feature = "argb")]
impl<T> ARGB<T> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    #[deprecated(note="This function has a misleading order of arguments. Use ARGB{} literal instead")]
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self {r,g,b,a}
    }
}

#[cfg(feature = "argb")]
impl<T, A> ARGB<T,A> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    #[deprecated(note="This function has a misleading order of arguments. Use ARGB{} literal instead")]
    pub const fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
        Self {r,g,b,a}
    }
}

#[cfg(feature = "argb")]
impl<T> ABGR<T> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    #[deprecated(note="This function has a misleading order of arguments. Use ABGR{} literal instead")]
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self {r,g,b,a}
    }
}

#[cfg(feature = "argb")]
impl<T, A> ABGR<T,A> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    #[deprecated(note="This function has a misleading order of arguments. Use ABGR{} literal instead")]
    pub const fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
        Self {r,g,b,a}
    }
}

macro_rules! impl_rgba {
    ($RGBA:ident) => {
        impl<T: Clone> $RGBA<T> {
            /// Iterate over all components (length=4)
            #[inline(always)]
            pub fn iter(&self) -> core::iter::Cloned<core::slice::Iter<'_, T>> {
                self.as_slice().iter().cloned()
            }
        }

        impl<T: Clone, A> $RGBA<T, A> {
            /// Copy RGB components out of the RGBA struct
            ///
            /// Note: you can use `.into()` to convert between other types
            #[inline(always)]
            pub fn bgr(&self) -> BGR<T> {
                BGR {r:self.r.clone(), g:self.g.clone(), b:self.b.clone()}
            }
        }

        impl<T: Copy, A: Clone> $RGBA<T, A> {
            /// Create new RGBA with the same alpha value, but different RGB values
            #[inline(always)]
            pub fn map_rgb<F, U, B>(&self, mut f: F) -> $RGBA<U, B>
                where F: FnMut(T) -> U, U: Clone, B: From<A> + Clone
            {
                $RGBA {
                    r: f(self.r),
                    g: f(self.g),
                    b: f(self.b),
                    a: self.a.clone().into(),
                }
            }

            #[inline(always)]
            /// Create a new RGBA with the new alpha value, but same RGB values
            pub fn alpha(&self, a: A) -> Self {
                Self {
                    r: self.r, g: self.g, b: self.b, a,
                }
            }

            /// Create a new RGBA with a new alpha value created by the callback.
            /// Allows changing of the type used for the alpha channel.
            #[inline]
            pub fn map_alpha<F, B>(&self, f: F) -> $RGBA<T, B>
                where F: FnOnce(A) -> B {
                $RGBA {
                    r: self.r,
                    g: self.g,
                    b: self.b,
                    a: f(self.a.clone()),
                }
            }
        }

        impl<T: Copy, B> ComponentMap<$RGBA<B>, T, B> for $RGBA<T> {
            #[inline(always)]
            fn map<F>(&self, mut f: F) -> $RGBA<B>
            where
                F: FnMut(T) -> B,
            {
                $RGBA {
                    r: f(self.r),
                    g: f(self.g),
                    b: f(self.b),
                    a: f(self.a),
                }
            }
        }

        impl<T: Copy, A: Copy, B> ColorComponentMap<$RGBA<B, A>, T, B> for $RGBA<T, A> {
            #[inline(always)]
            fn map_c<F>(&self, mut f: F) -> $RGBA<B, A>
            where
                F: FnMut(T) -> B,
            {
                $RGBA {
                    r: f(self.r),
                    g: f(self.g),
                    b: f(self.b),
                    a: self.a,
                }
            }
        }

        impl<T> ComponentSlice<T> for $RGBA<T> {
            #[inline(always)]
            fn as_slice(&self) -> &[T] {
                unsafe {
                    core::slice::from_raw_parts(self as *const Self as *const T, 4)
                }
            }

            #[inline(always)]
            fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    core::slice::from_raw_parts_mut(self as *mut Self as *mut T, 4)
                }
            }
        }

        impl<T> ComponentSlice<T> for [$RGBA<T>] {
            #[inline]
            fn as_slice(&self) -> &[T] {
                unsafe {
                    core::slice::from_raw_parts(self.as_ptr() as *const _, self.len() * 4)
                }
            }
            #[inline]
            fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() * 4)
                }
            }
        }

        #[cfg(feature = "as-bytes")]
        impl<T: crate::Pod> ComponentBytes<T> for [$RGBA<T>] {}
    }
}

macro_rules! impl_alpha_conv {
    ($RGB:ident, $RGBA:ident) => {
        /// Assumes 255 is opaque
        impl<T: Copy> From<$RGB<T>> for $RGBA<T, u8> {
            #[inline(always)]
            fn from(other: $RGB<T>) -> Self {
                Self {
                    r: other.r,
                    g: other.g,
                    b: other.b,
                    a: 0xFF,
                }
            }
        }

        /// Assumes 65535 is opaque
        impl<T: Copy> From<$RGB<T>> for $RGBA<T, u16> {
            #[inline(always)]
            fn from(other: $RGB<T>) -> Self {
                Self {
                    r: other.r,
                    g: other.g,
                    b: other.b,
                    a: 0xFFFF,
                }
            }
        }
    }
}

impl<T, A> RGBA<T, A> {
    /// Provide a mutable view of only RGB components (leaving out alpha).
    /// Useful to change color without changing opacity.
    #[inline(always)]
    pub fn rgb_mut(&mut self) -> &mut RGB<T> {
        unsafe {
            &mut *(self as *mut _ as *mut RGB<T>)
        }
    }
}

impl<T, A> BGRA<T, A> {
    /// Provide a mutable view of only RGB components (leaving out alpha).
    /// Useful to change color without changing opacity.
    #[inline(always)]
    #[deprecated(note = "This function will change. Use bgr_mut()")]
    pub fn rgb_mut(&mut self) -> &mut BGR<T> {
        unsafe {
            &mut *(self as *mut _ as *mut BGR<T>)
        }
    }

    /// Provide a mutable view of only RGB components (leaving out alpha).
    /// Useful to change color without changing opacity.
    #[inline(always)]
    pub fn bgr_mut(&mut self) -> &mut BGR<T> {
        unsafe {
            &mut *(self as *mut _ as *mut BGR<T>)
        }
    }
}

impl<T> core::iter::FromIterator<T> for RGBA<T> {
    #[inline(always)]
    /// Takes exactly 4 elements from the iterator and creates a new instance.
    /// Panics if there are fewer elements in the iterator.
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
        let mut iter = into_iter.into_iter();
        Self {
            r: iter.next().unwrap(),
            g: iter.next().unwrap(),
            b: iter.next().unwrap(),
            a: iter.next().unwrap(),
        }
    }
}

impl<T: Clone, A> RGBA<T, A> {
    /// Copy RGB components out of the RGBA struct
    ///
    /// Note: you can use `.into()` to convert between other types
    #[inline(always)]
    pub fn rgb(&self) -> RGB<T> {
        RGB {r:self.r.clone(), g:self.g.clone(), b:self.b.clone()}
    }
}

#[cfg(feature = "argb")]
impl<T: Clone, A> ARGB<T, A> {
    /// Copy RGB components out of the ARGB struct
    ///
    /// Note: you can use `.into()` to convert between other types
    #[inline(always)]
    pub fn rgb(&self) -> RGB<T> {
        RGB {r:self.r.clone(), g:self.g.clone(), b:self.b.clone()}
    }
}

impl<T: Clone, A> BGRA<T, A> {
    /// Copy RGB components out of the RGBA struct
    ///
    /// Note: you can use `.into()` to convert between other types
    #[inline(always)]
    #[deprecated(note = "This function will change. Use bgr()")]
    pub fn rgb(&self) -> BGR<T> {
        BGR {r:self.r.clone(), g:self.g.clone(), b:self.b.clone()}
    }
}

impl_rgba! {RGBA}
impl_rgba! {BGRA}
#[cfg(feature = "argb")]
impl_rgba! {ARGB}
#[cfg(feature = "argb")]
impl_rgba! {ABGR}

impl_alpha_conv! {BGR, BGRA}
impl_alpha_conv! {RGB, BGRA}
impl_alpha_conv! {BGR, RGBA}
impl_alpha_conv! {RGB, RGBA}
#[cfg(feature = "argb")]
impl_alpha_conv! {BGR, ABGR}
#[cfg(feature = "argb")]
impl_alpha_conv! {RGB, ABGR}
#[cfg(feature = "argb")]
impl_alpha_conv! {BGR, ARGB}
#[cfg(feature = "argb")]
impl_alpha_conv! {RGB, ARGB}
