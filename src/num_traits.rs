use crate::{Abgr, Argb, Bgr, Bgra, GrayA, Gray_v09, Grb, Rgb, Rgba, Rgbw};

/// Re-exports from [the `num-traits` crate](https://lib.rs/crates/num-traits).
pub use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
pub use num_traits::ops::saturating::{SaturatingAdd, SaturatingMul, SaturatingSub};

macro_rules! num_traits_without_alpha {
($name:ident, [$($bit:tt),*]) => {
    impl<T: CheckedAdd> CheckedAdd for $name<T> {
        #[inline(always)]
        fn checked_add(&self, other: &Self) -> Option<Self> {
            Some(Self {
                $(
                    $bit: self.$bit.checked_add(&other.$bit)?,
                )+
            })
        }
    }

    impl<T: CheckedSub> CheckedSub for $name<T> {
        #[inline(always)]
        fn checked_sub(&self, other: &Self) -> Option<Self> {
            Some(Self {
                $(
                    $bit: self.$bit.checked_sub(&other.$bit)?,
                )+
            })
        }
    }

    impl<T: CheckedMul> CheckedMul for $name<T> {
        #[inline(always)]
        fn checked_mul(&self, other: &Self) -> Option<Self> {
            Some(Self {
                $(
                    $bit: self.$bit.checked_mul(&other.$bit)?,
                )+
            })
        }
    }

    impl<T: CheckedDiv> CheckedDiv for $name<T> {
        #[inline(always)]
        fn checked_div(&self, other: &Self) -> Option<Self> {
            Some(Self {
                $(
                    $bit: self.$bit.checked_div(&other.$bit)?,
                )+
            })
        }
    }

    impl<T: SaturatingAdd> SaturatingAdd for $name<T> {
        #[inline(always)]
        fn saturating_add(&self, other: &Self) -> Self {
            Self {
                $(
                    $bit: self.$bit.saturating_add(&other.$bit),
                )+
            }
        }
    }

    impl<T: SaturatingSub> SaturatingSub for $name<T> {
        #[inline(always)]
        fn saturating_sub(&self, other: &Self) -> Self {
            Self {
                $(
                    $bit: self.$bit.saturating_sub(&other.$bit),
                )+
            }
        }
    }

    impl<T: SaturatingMul> SaturatingMul for $name<T> {
        #[inline(always)]
        fn saturating_mul(&self, other: &Self) -> Self {
            Self {
                $(
                    $bit: self.$bit.saturating_mul(&other.$bit),
                )+
            }
        }
    }
};
}

macro_rules! num_traits_with_alpha {
($name:ident, [$($bit:tt),*]) => {
    impl<T: CheckedSub, A: CheckedSub> CheckedSub for $name<T, A> {
        #[inline(always)]
        fn checked_sub(&self, other: &Self) -> Option<Self> {
            Some(Self {
                $(
                    $bit: self.$bit.checked_sub(&other.$bit)?,
                )+
            })
        }
    }

    impl<T: CheckedMul, A: CheckedMul> CheckedMul for $name<T, A> {
        #[inline(always)]
        fn checked_mul(&self, other: &Self) -> Option<Self> {
            Some(Self {
                $(
                    $bit: self.$bit.checked_mul(&other.$bit)?,
                )+
            })
        }
    }

    impl<T: CheckedDiv, A: CheckedDiv> CheckedDiv for $name<T, A> {
        #[inline(always)]
        fn checked_div(&self, other: &Self) -> Option<Self> {
            Some(Self {
                $(
                    $bit: self.$bit.checked_div(&other.$bit)?,
                )+
            })
        }
    }

    impl<T: SaturatingAdd, A: SaturatingAdd> SaturatingAdd for $name<T, A> {
        #[inline(always)]
        fn saturating_add(&self, other: &Self) -> Self {
            Self {
                $(
                    $bit: self.$bit.saturating_add(&other.$bit),
                )+
            }
        }
    }

    impl<T: SaturatingSub, A: SaturatingSub> SaturatingSub for $name<T, A> {
        #[inline(always)]
        fn saturating_sub(&self, other: &Self) -> Self {
            Self {
                $(
                    $bit: self.$bit.saturating_sub(&other.$bit),
                )+
            }
        }
    }

    impl<T: SaturatingMul, A: SaturatingMul> SaturatingMul for $name<T, A> {
        #[inline(always)]
        fn saturating_mul(&self, other: &Self) -> Self {
            Self {
                $(
                    $bit: self.$bit.saturating_mul(&other.$bit),
                )+
            }
        }
    }
};
}

num_traits_without_alpha!(Rgb, [r, g, b]);
num_traits_without_alpha!(Bgr, [b, g, r]);
num_traits_without_alpha!(Grb, [g, r, b]);
num_traits_without_alpha!(Gray_v09, [v]);
num_traits_without_alpha!(Rgbw, [r, g, b, w]);

num_traits_with_alpha!(Rgba, [r, g, b, a]);
num_traits_with_alpha!(Argb, [a, r, g, b]);
num_traits_with_alpha!(Bgra, [b, g, r, a]);
num_traits_with_alpha!(Abgr, [a, b, g, r]);
num_traits_with_alpha!(GrayA, [v, a]);

#[test]
#[cfg(not(feature = "checked_fns"))]
fn test_checked_sub() {
    assert_eq!(
        Rgba::<u8>::new(2, 4, 6, 111).checked_sub(&Rgba::<u8>::new(3, 4, 6, 0)),
        None
    );
    assert_eq!(
        Rgb::<u8>::new(2, 4, 6).checked_sub(&Rgb::<u8>::new(2, 5, 6)),
        None
    );
    assert_eq!(
        Rgb::<u8>::new(2, 4, 6).checked_sub(&Rgb::<u8>::new(2, 4, 7)),
        None
    );
    assert_eq!(
        Rgb::<u8>::new(2, 4, 6).checked_sub(&Rgb::<u8>::new(2, 4, 6)),
        Some([0, 0, 0].into())
    );

    assert_eq!(
        Rgb::<i8>::new(-128, 4, 6).checked_sub(&Rgb::<i8>::new(1, 4, 7)),
        None
    );
    assert_eq!(
        Rgb::<i8>::new(2, -128, 6).checked_sub(&Rgb::<i8>::new(2, 1, 7)),
        None
    );
    assert_eq!(
        Rgb::<i8>::new(2, 4, -128).checked_sub(&Rgb::<i8>::new(2, 4, 1)),
        None
    );
    assert_eq!(
        Rgb::<i8>::new(2, 4, 6).checked_sub(&Rgb::<i8>::new(-2, 4, 6)),
        Some(Rgb::<i8>::new(4, 0, 0))
    );
}
