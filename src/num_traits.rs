use crate::{Abgr, Argb, Bgr, Bgra, Gray, GrayA, Grb, Rgb, Rgba};

use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

macro_rules! num_traits {
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
};
}

num_traits!(Rgb, [r, g, b]);
num_traits!(Bgr, [b, g, r]);
num_traits!(Grb, [g, r, b]);
num_traits!(Rgba, [r, g, b, a]);
num_traits!(Argb, [a, r, g, b]);
num_traits!(Bgra, [b, g, r, a]);
num_traits!(Abgr, [a, b, g, r]);
num_traits!(Gray, [0]);
num_traits!(GrayA, [0, 1]);
