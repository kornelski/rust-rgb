use crate::{Abgr, Argb, Bgr, Bgra, Gray, GrayAlpha, Grb, Rgb, Rgba};

macro_rules! impl_rgb {
    ($Rgb:ident) => {
        impl<T: Clone> $Rgb<T> {
            /// Iterate over all components (length=4)
            #[inline(always)]
            pub fn iter(&self) -> core::iter::Cloned<core::slice::Iter<'_, T>> {
                unsafe { core::slice::from_raw_parts(self as *const Self as *const T, 3) }
                    .iter()
                    .cloned()
            }
        }
    };
}

macro_rules! impl_rgba {
    ($Rgba:ident) => {
        impl<T: Clone> $Rgba<T> {
            /// Iterate over all components (length=4)
            #[inline(always)]
            pub fn iter(&self) -> core::iter::Cloned<core::slice::Iter<'_, T>> {
                unsafe { core::slice::from_raw_parts(self as *const Self as *const T, 4) }
                    .iter()
                    .cloned()
            }
        }

        impl<T: Clone, A> $Rgba<T, A> {
            /// Copy Rgb components out of the Rgba struct
            ///
            /// Note: you can use `.into()` to convert between other types
            #[inline(always)]
            pub fn bgr(&self) -> Bgr<T> {
                Bgr {
                    r: self.r.clone(),
                    g: self.g.clone(),
                    b: self.b.clone(),
                }
            }
        }

        impl<T: Copy, A: Clone> $Rgba<T, A> {
            /// Create new Rgba with the same alpha value, but different Rgb values
            #[inline(always)]
            pub fn map_rgb<F, U, B>(&self, mut f: F) -> $Rgba<U, B>
            where
                F: FnMut(T) -> U,
                U: Clone,
                B: From<A> + Clone,
            {
                $Rgba {
                    r: f(self.r),
                    g: f(self.g),
                    b: f(self.b),
                    a: self.a.clone().into(),
                }
            }

            #[inline(always)]
            /// Create a new Rgba with the new alpha value, but same Rgb values
            pub fn alpha(&self, a: A) -> Self {
                Self {
                    r: self.r,
                    g: self.g,
                    b: self.b,
                    a,
                }
            }

            /// Create a new Rgba with a new alpha value created by the callback.
            /// Allows changing of the type used for the alpha channel.
            #[inline]
            pub fn map_alpha<F, B>(&self, f: F) -> $Rgba<T, B>
            where
                F: FnOnce(A) -> B,
            {
                $Rgba {
                    r: self.r,
                    g: self.g,
                    b: self.b,
                    a: f(self.a.clone()),
                }
            }
        }
    };
}

macro_rules! impl_rgb_to_alpha {
    ($Rgb:ident, $Rgba:ident) => {
        impl<T: Clone> $Rgb<T> {
            /// Convenience function for converting to Rgba
            #[inline(always)]
            pub fn alpha(&self, a: T) -> $Rgba<T> {
                $Rgba {
                    r: self.r.clone(),
                    g: self.g.clone(),
                    b: self.b.clone(),
                    a,
                }
            }

            /// Convenience function for converting to Rgba with alpha channel of a different type than type of the pixels
            #[inline(always)]
            pub fn new_alpha<A>(&self, a: A) -> $Rgba<T, A> {
                $Rgba {
                    r: self.r.clone(),
                    g: self.g.clone(),
                    b: self.b.clone(),
                    a,
                }
            }
        }
    };
}

macro_rules! new_with_alpha {
    (dep, $name:ident) => {
        impl<T> $name<T> {
            /// Convenience function for creating a new pixel
            /// Warning: The order of arguments is R,G,B,A
            #[deprecated(note="This function has a misleading order of arguments. Use Struct{} literal instead")]
            pub const fn new(r: T, g: T, b: T, a: T) -> Self {
                Self {r,g,b,a}
            }
        }
        impl<T, A> $name<T, A> {
            /// Convenience function for creating a new pixel
            /// Warning: The order of arguments is R,G,B,A
            #[deprecated(note="This function has a misleading order of arguments. Use Struct{} literal instead")]
            pub const fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
                Self {r,g,b,a}
            }
        }
    };
    (not_dep, $name:ident) => {
        impl<T> $name<T> {
            /// Convenience function for creating a new pixel
            pub const fn new(r: T, g: T, b: T, a: T) -> Self {
                Self {r,g,b,a}
            }
        }
        impl<T, A> $name<T, A> {
            /// Convenience function for creating a new pixel
            pub const fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
                Self {r,g,b,a}
            }
        }
    };
}
macro_rules! new_without_alpha {
    (dep, $name:ident) => {
        impl<T> $name<T> {
            /// Convenience function for creating a new pixel
            /// Warning: The order of arguments is R,G,B,A
            #[deprecated(note="This function has a misleading order of arguments. Use Struct{} literal instead")]
            pub const fn new(r: T, g: T, b: T) -> Self {
                Self {r,g,b}
            }
        }
    };
    (not_dep, $name:ident) => {
        impl<T> $name<T> {
            /// Convenience function for creating a new pixel
            pub const fn new(r: T, g: T, b: T) -> Self {
                Self {r,g,b}
            }
        }
    };
}

impl<T> Gray<T> {
    /// New grayscale pixel
    #[inline(always)]
    pub const fn new(brightness: T) -> Self {
        Self(brightness)
    }
}

impl<T: Clone, A> GrayAlpha<T, A> {
    /// Copy `Gray` component out of the `GrayAlpha` struct
    #[inline(always)]
    pub fn gray(&self) -> Gray<T> {
        Gray(self.0.clone())
    }
}

impl<T, A> GrayAlpha<T, A> {
    /// New grayscale+alpha pixel
    #[inline(always)]
    pub const fn new(brightness: T, alpha: A) -> Self {
        Self(brightness, alpha)
    }

    /// Provide a mutable view of only `Gray` component (leaving out alpha).
    #[inline(always)]
    pub fn gray_mut(&mut self) -> &mut Gray<T> {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }
}

impl<T: Copy, A: Clone> GrayAlpha<T, A> {
    /// Create a new `GrayAlpha` with the new alpha value, but same gray value
    #[inline(always)]
    pub fn alpha(&self, a: A) -> Self {
        Self(self.0, a)
    }

    /// Create a new `GrayAlpha` with a new alpha value created by the callback.
    #[inline(always)]
    pub fn map_alpha<F, B>(&self, f: F) -> GrayAlpha<T, B>
    where
        F: FnOnce(A) -> B,
    {
        GrayAlpha(self.0, f(self.1.clone()))
    }

    /// Create new `GrayAlpha` with the same alpha value, but different `Gray` value
    #[inline(always)]
    pub fn map_gray<F, U, B>(&self, f: F) -> GrayAlpha<U, B>
    where
        F: FnOnce(T) -> U,
        U: Clone,
        B: From<A> + Clone,
    {
        GrayAlpha(f(self.0), self.1.clone().into())
    }
}

macro_rules! impl_alpha_conv {
    ($Rgb:ident, $Rgba:ident) => {
        /// Assumes 255 is opaque
        impl<T: Copy> From<$Rgb<T>> for $Rgba<T, u8> {
            #[inline(always)]
            fn from(other: $Rgb<T>) -> Self {
                Self {
                    r: other.r,
                    g: other.g,
                    b: other.b,
                    a: 0xFF,
                }
            }
        }

        /// Assumes 65535 is opaque
        impl<T: Copy> From<$Rgb<T>> for $Rgba<T, u16> {
            #[inline(always)]
            fn from(other: $Rgb<T>) -> Self {
                Self {
                    r: other.r,
                    g: other.g,
                    b: other.b,
                    a: 0xFFFF,
                }
            }
        }
    };
}

impl<T, A> Rgba<T, A> {
    /// Provide a mutable view of only Rgb components (leaving out alpha).
    /// Useful to change color without changing opacity.
    #[inline(always)]
    pub fn rgb_mut(&mut self) -> &mut Rgb<T> {
        unsafe { &mut *(self as *mut _ as *mut Rgb<T>) }
    }
}

impl<T, A> Bgra<T, A> {
    /// Provide a mutable view of only Rgb components (leaving out alpha).
    /// Useful to change color without changing opacity.
    #[inline(always)]
    #[deprecated(note = "This function will change. Use bgr_mut()")]
    pub fn rgb_mut(&mut self) -> &mut Bgr<T> {
        unsafe { &mut *(self as *mut _ as *mut Bgr<T>) }
    }

    /// Provide a mutable view of only Rgb components (leaving out alpha).
    /// Useful to change color without changing opacity.
    #[inline(always)]
    pub fn bgr_mut(&mut self) -> &mut Bgr<T> {
        unsafe { &mut *(self as *mut _ as *mut Bgr<T>) }
    }
}

impl<T> core::iter::FromIterator<T> for Rgb<T> {
    #[inline(always)]
    /// Takes exactly 3 elements from the iterator and creates a new instance.
    /// Panics if there are fewer elements in the iterator.
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
        let mut iter = into_iter.into_iter();
        Self {
            r: iter.next().unwrap(),
            g: iter.next().unwrap(),
            b: iter.next().unwrap(),
        }
    }
}
impl<T> core::iter::FromIterator<T> for Rgba<T> {
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

impl<T: Clone, A> Rgba<T, A> {
    /// Copy Rgb components out of the Rgba struct
    ///
    /// Note: you can use `.into()` to convert between other types
    #[inline(always)]
    pub fn rgb(&self) -> Rgb<T> {
        Rgb {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
        }
    }
}

impl<T: Clone, A> Argb<T, A> {
    /// Copy Rgb components out of the ARgb struct
    ///
    /// Note: you can use `.into()` to convert between other types
    #[inline(always)]
    pub fn rgb(&self) -> Rgb<T> {
        Rgb {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
        }
    }
}

impl<T: Clone, A> Bgra<T, A> {
    /// Copy Rgb components out of the Rgba struct
    ///
    /// Note: you can use `.into()` to convert between other types
    #[inline(always)]
    #[deprecated(note = "This function will change. Use bgr()")]
    pub fn rgb(&self) -> Bgr<T> {
        Bgr {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
        }
    }
}

impl_rgb!(Rgb);
impl_rgb!(Bgr);
impl_rgb!(Grb);

impl_rgba!(Rgba);
impl_rgba!(Argb);
impl_rgba!(Bgra);
impl_rgba!(Abgr);

impl_rgb_to_alpha! {Rgb, Rgba}
impl_rgb_to_alpha! {Bgr, Bgra}

new_with_alpha!(not_dep, Rgba);
new_with_alpha!(dep, Argb);
new_with_alpha!(dep, Bgra);
new_with_alpha!(dep, Abgr);
new_without_alpha!(not_dep, Rgb);
new_without_alpha!(dep, Bgr);

impl_alpha_conv! {Bgr, Bgra}
impl_alpha_conv! {Rgb, Bgra}
impl_alpha_conv! {Bgr, Rgba}
impl_alpha_conv! {Rgb, Rgba}
impl_alpha_conv! {Bgr, Abgr}
impl_alpha_conv! {Rgb, Abgr}
impl_alpha_conv! {Bgr, Argb}
impl_alpha_conv! {Rgb, Argb}
