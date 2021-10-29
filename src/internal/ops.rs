use crate::alt::Gray;
use crate::alt::GrayAlpha;
use super::pixel::*;
use crate::RGB;
use crate::RGBA;
use core::ops::*;
use core::iter::Sum;
#[cfg(feature = "argb")]
use crate::alt::ARGB;

macro_rules! impl_struct_ops_opaque {
    ($ty:ident => $($field:tt)+) => {
        /// `px + px`
        impl<T: Add> Add for $ty<T> {
            type Output = $ty<<T as Add>::Output>;

            #[inline(always)]
            fn add(self, other: $ty<T>) -> Self::Output {
                $ty {
                    $(
                        $field: self.$field + other.$field,
                    )+
                }
            }
        }

        /// `px + px`
        impl<T> AddAssign for $ty<T> where
            T: Add<Output = T> + Copy
        {
            #[inline(always)]
            fn add_assign(&mut self, other: $ty<T>) {
                *self = Self {
                    $(
                        $field: self.$field + other.$field,
                    )+
                };
            }
        }

        /// `px - px`
        impl<T: Sub> Sub for $ty<T> {
            type Output = $ty<<T as Sub>::Output>;

            #[inline(always)]
            fn sub(self, other: $ty<T>) -> Self::Output {
                $ty {
                    $(
                        $field: self.$field - other.$field,
                    )+
                }
            }
        }

        /// `px - px`
        impl<T> SubAssign for $ty<T> where
            T: Sub<Output = T> + Copy
        {
            #[inline(always)]
            fn sub_assign(&mut self, other: $ty<T>) {
                *self = Self {
                    $(
                        $field: self.$field - other.$field,
                    )+
                };
            }
        }

        impl<T> Sum<$ty<T>> for $ty<T> where T: Default + Add<Output=T> {
            #[inline(always)]
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold($ty::default(), Add::add)
            }
        }
    };
}

macro_rules! impl_struct_ops_alpha {
    ($ty:ident => $($field:tt)+) => {
        /// `px + px`
        impl<T: Add, A: Add> Add for $ty<T, A> {
            type Output = $ty<<T as Add>::Output, <A as Add>::Output>;

            #[inline(always)]
            fn add(self, other: $ty<T, A>) -> Self::Output {
                $ty {
                    $(
                        $field: self.$field + other.$field,
                    )+
                }
            }
        }

        /// `px + px`
        impl<T, A> AddAssign for $ty<T, A> where
            T: Add<Output = T> + Copy,
            A: Add<Output = A> + Copy
        {
            #[inline(always)]
            fn add_assign(&mut self, other: $ty<T, A>) {
                *self = Self {
                    $(
                        $field: self.$field + other.$field,
                    )+
                };
            }
        }

        /// `px - px`
        impl<T: Sub, A: Sub> Sub for $ty<T, A> {
            type Output = $ty<<T as Sub>::Output, <A as Sub>::Output>;

            #[inline(always)]
            fn sub(self, other: $ty<T, A>) -> Self::Output {
                $ty {
                    $(
                        $field: self.$field - other.$field,
                    )+
                }
            }
        }

        /// `px - px`
        impl<T, A> SubAssign for $ty<T, A> where
            T: Sub<Output = T> + Copy,
            A: Sub<Output = A> + Copy
        {
            #[inline(always)]
            fn sub_assign(&mut self, other: $ty<T, A>) {
                *self = Self {
                    $(
                        $field: self.$field - other.$field,
                    )+
                };
            }
        }

        impl<T, A> Sum<$ty<T, A>> for $ty<T, A> where T: Default + Add<Output=T>, A: Default + Add<Output=A> {
            #[inline(always)]
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold($ty::default(), Add::add)
            }
        }
    };
}

macro_rules! impl_scalar {
    ($ty:ident) => {
        /// `px - 1`
        impl<T> Sub<T> for $ty<T> where
            T: Copy + Sub<Output=T>
        {
            type Output = $ty<<T as Sub>::Output>;

            #[inline(always)]
            fn sub(self, r: T) -> Self::Output {
                self.map(|l| l-r)
            }
        }

        /// `px - 1`
        impl<T> SubAssign<T> for $ty<T> where
            T: Copy + Sub<Output=T>
        {
            #[inline(always)]
            fn sub_assign(&mut self, r: T) {
                *self = self.map(|l| l-r);
            }
        }

        /// `px + 1`
        impl<T> Add<T> for $ty<T> where
            T: Copy + Add<Output=T>
        {
            type Output = $ty<T>;

            #[inline(always)]
            fn add(self, r: T) -> Self::Output {
                self.map(|l|l+r)
            }
        }

        /// `px + 1`
        impl<T> AddAssign<T> for $ty<T> where
            T: Copy + Add<Output=T>
        {
            #[inline(always)]
            fn add_assign(&mut self, r: T) {
                *self = self.map(|l| l+r);
            }
        }

        /// `px * 1`
        impl<T> Mul<T> for $ty<T> where
            T: Copy + Mul<Output=T>
        {
            type Output = $ty<T>;

            #[inline(always)]
            fn mul(self, r: T) -> Self::Output {
                self.map(|l|l*r)
            }
        }

        /// `px * 1`
        impl<T> MulAssign<T> for $ty<T> where
            T: Copy + Mul<Output=T>
        {
            #[inline(always)]
            fn mul_assign(&mut self, r: T) {
                *self = self.map(|l| l*r);
            }
        }

        /// `px / 1`
        impl<T> Div<T> for $ty<T> where
            T: Copy + Div<Output=T>
        {
            type Output = $ty<T>;

            #[inline(always)]
            fn div(self, r: T) -> Self::Output {
                self.map(|l| l / r)
            }
        }

        /// `px * 1`
        impl<T> DivAssign<T> for $ty<T> where
            T: Copy + Div<Output=T>
        {
            #[inline(always)]
            fn div_assign(&mut self, r: T) {
                *self = self.map(|l| l / r);
            }
        }
    }
}

impl_scalar!{RGB}
impl_scalar!{RGBA}
#[cfg(feature = "argb")]
impl_scalar!{ARGB}
impl_scalar!{Gray}
impl_scalar!{GrayAlpha}

impl_struct_ops_opaque! {RGB => r g b}
impl_struct_ops_opaque! {Gray => 0}

impl_struct_ops_alpha! {RGBA => r g b a}
#[cfg(feature = "argb")]
impl_struct_ops_alpha! {ARGB => a r g b}
impl_struct_ops_alpha! {GrayAlpha => 0 1}

#[cfg(test)]
mod test {
    use super::*;
    const WHITE_RGB: RGB<u8> = RGB::new(255, 255, 255);
    const BLACK_RGB: RGB<u8> = RGB::new(0, 0, 0);
    const RED_RGB: RGB<u8> = RGB::new(255, 0, 0);
    const GREEN_RGB: RGB<u8> = RGB::new(0, 255, 0);
    const BLUE_RGB: RGB<u8> = RGB::new(0, 0, 255);

    const WHITE_RGBA: RGBA<u8> = RGBA::new(255, 255, 255, 255);
    const BLACK_RGBA: RGBA<u8> = RGBA::new(0, 0, 0, 0);
    const RED_RGBA: RGBA<u8> = RGBA::new(255, 0, 0, 255);
    const GREEN_RGBA: RGBA<u8> = RGBA::new(0, 255, 0, 0);
    const BLUE_RGBA: RGBA<u8> = RGBA::new(0, 0, 255, 255);

    #[test]
    fn test_add() {
        assert_eq!(RGB::new(2,4,6), RGB::new(1,2,3) + RGB{r:1,g:2,b:3});
        assert_eq!(RGB::new(2.,4.,6.), RGB::new(1.,3.,5.) + 1.);

        assert_eq!(RGBA::new_alpha(2u8,4,6,8u16), RGBA::new_alpha(1u8,2,3,4u16) + RGBA{r:1u8,g:2,b:3,a:4u16});
        assert_eq!(RGBA::new(2i16,4,6,8), RGBA::new(1,3,5,7) + 1);

        assert_eq!(RGB::new(255, 255, 0), RED_RGB+GREEN_RGB);
        assert_eq!(RGB::new(255, 0, 0), RED_RGB+RGB::new(0, 0, 0));
        assert_eq!(WHITE_RGB, BLACK_RGB + 255);

        assert_eq!(RGBA::new(255, 255, 0, 255), RED_RGBA+GREEN_RGBA);
        assert_eq!(RGBA::new(255, 0, 0, 255), RED_RGBA+RGBA::new(0, 0, 0, 0));
        assert_eq!(WHITE_RGBA, BLACK_RGBA + 255);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn test_add_overflow() {
        assert_ne!(RGBA::new(255u8, 255, 0, 0), RED_RGBA+BLUE_RGBA);
    }

    #[test]
    fn test_sub() {
        assert_eq!(RED_RGB, (WHITE_RGB - GREEN_RGB) - BLUE_RGB);
        assert_eq!(BLACK_RGB, WHITE_RGB - 255);

        assert_eq!(RGBA::new(255, 255, 0, 0), WHITE_RGBA - BLUE_RGBA);
        assert_eq!(BLACK_RGBA, WHITE_RGBA - 255);
    }

    #[test]
    fn test_add_assign() {
        let mut green_rgb = RGB::new(0, 255, 0);
        green_rgb += RGB::new(255, 0, 255);
        assert_eq!(WHITE_RGB, green_rgb);

        let mut black_rgb = RGB::new(0, 0, 0);
        black_rgb += 255;
        assert_eq!(WHITE_RGB, black_rgb);

        let mut green_rgba = RGBA::new(0, 255, 0, 0);
        green_rgba += RGBA::new(255, 0, 255, 255);
        assert_eq!(WHITE_RGBA, green_rgba);

        let mut black_rgba = RGBA::new(0, 0, 0, 0);
        black_rgba += 255;
        assert_eq!(WHITE_RGBA, black_rgba);
    }

    #[test]
    fn test_sub_assign() {
        let mut green_rgb = RGB::new(0, 255, 0);
        green_rgb -= RGB::new(0, 255, 0);
        assert_eq!(BLACK_RGB, green_rgb);

        let mut white_rgb = RGB::new(255, 255, 255);
        white_rgb -= 255;
        assert_eq!(BLACK_RGB, white_rgb);

        let mut green_rgba = RGBA::new(0, 255, 0, 0);
        green_rgba -= RGBA::new(0, 255, 0, 0);
        assert_eq!(BLACK_RGBA, green_rgba);

        let mut white_rgba = RGBA::new(255, 255, 255, 255);
        white_rgba -= 255;
        assert_eq!(BLACK_RGBA, white_rgba);
    }

    #[test]
    fn test_mult() {
        assert_eq!(RGB::new(0.5,1.5,2.5), RGB::new(1.,3.,5.) * 0.5);
        assert_eq!(RGBA::new(2,4,6,8), RGBA::new(1,2,3,4) * 2);
    }

    #[test]
    fn test_mult_assign() {
        let mut green_rgb = RGB::new(0u16, 255, 0);
        green_rgb *= 1;
        assert_eq!(RGB::new(0, 255, 0), green_rgb);
        green_rgb *= 2;
        assert_eq!(RGB::new(0, 255*2, 0), green_rgb);

        let mut green_rgba = RGBA::new(0u16, 255, 0, 0);
        green_rgba *= 1;
        assert_eq!(RGBA::new(0, 255, 0, 0), green_rgba);
        green_rgba *= 2;
        assert_eq!(RGBA::new(0, 255*2, 0, 0), green_rgba);
    }

    #[test]
    fn sum() {
        let s1 = [RGB::new(1u8,1,1), RGB::new(2,3,4)].iter().copied().sum::<RGB<u8>>();
        let s2 = [RGB::new(1u16,1,1), RGB::new(2,3,4)].iter().copied().sum::<RGB<u16>>();
        let s3 = [RGBA::new_alpha(1u8,1,1,1u16), RGBA::new_alpha(2,3,4,5)].iter().copied().sum::<RGBA<u8, u16>>();
        let s4 = [RGBA::new_alpha(1u16,1,1,1u8), RGBA::new_alpha(2,3,4,5)].iter().copied().sum::<RGBA<u16, u8>>();
        assert_eq!(s1, RGB::new(3, 4, 5));
        assert_eq!(s2, RGB::new(3, 4, 5));
        assert_eq!(s3, RGBA::new_alpha(3, 4, 5, 6));
        assert_eq!(s4, RGBA::new_alpha(3, 4, 5, 6));
    }
}
