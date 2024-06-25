use crate::{Abgr, Argb, Bgr, Bgra, Gray, GrayA, Grb, Rgb, Rgba, Rgbw};

use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

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
};
}

macro_rules! num_traits_with_alpha {
($name:ident, [$($bit:tt),*]) => {
    impl<T: CheckedAdd, A: CheckedAdd> CheckedAdd for $name<T, A> {
        #[inline(always)]
        fn checked_add(&self, other: &Self) -> Option<Self> {
            Some(Self {
                $(
                    $bit: self.$bit.checked_add(&other.$bit)?,
                )+
            })
        }
    }

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
};
}

num_traits_without_alpha!(Rgb, [r, g, b]);
num_traits_without_alpha!(Bgr, [b, g, r]);
num_traits_without_alpha!(Grb, [g, r, b]);
num_traits_without_alpha!(Gray, [v]);
num_traits_without_alpha!(Rgbw, [r, g, b, w]);

num_traits_with_alpha!(Rgba, [r, g, b, a]);
num_traits_with_alpha!(Argb, [a, r, g, b]);
num_traits_with_alpha!(Bgra, [b, g, r, a]);
num_traits_with_alpha!(Abgr, [a, b, g, r]);
num_traits_with_alpha!(GrayA, [v, a]);
