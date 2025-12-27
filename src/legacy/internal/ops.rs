#[allow(unused_imports)]
use crate::formats::gray::Gray_v08;
#[allow(unused_imports)]
use crate::formats::gray_alpha::GrayAlpha_v08;
#[allow(unused_imports)]
use crate::{Abgr, Argb, Bgr, Bgra, Grb, Rgb, Rgba};

#[allow(unused_macros)]
macro_rules! impl_struct_checked {
    ($ty:ident, $field_ty:ident, => $($field:tt)+) => {
        impl $ty<$field_ty>
        {
            /// `px.checked_add(px)`
            #[inline(always)]
            #[cfg_attr(feature = "num-traits", deprecated(note = "import `num_traits::CheckedAdd` instead, disable rgb's checked_fns feature"))]
            #[must_use] pub fn checked_add(self, rhs: $ty<$field_ty>) -> Option<Self> {
                Some($ty {
                    $(
                        $field: self.$field.checked_add(rhs.$field)?,
                    )+
                })
            }

            /// `px.checked_sub(px)`
            #[inline(always)]
            #[cfg_attr(feature = "num-traits", deprecated(note = "import `num_traits::CheckedSub` instead, disable rgb's checked_fns feature"))]
            #[must_use] pub fn checked_sub(self, rhs: $ty<$field_ty>) -> Option<Self> {
                Some($ty {
                    $(
                        $field: self.$field.checked_sub(rhs.$field)?,
                    )+
                })
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules! impl_legacy_checked {
    ($ty:ident => $($field:tt)+) => {
        impl_struct_checked!($ty, u8, => $($field)+);
        impl_struct_checked!($ty, u16, => $($field)+);
        impl_struct_checked!($ty, u32, => $($field)+);
        impl_struct_checked!($ty, u64, => $($field)+);
        impl_struct_checked!($ty, i8, => $($field)+);
        impl_struct_checked!($ty, i16, => $($field)+);
        impl_struct_checked!($ty, i32, => $($field)+);
        impl_struct_checked!($ty, i64, => $($field)+);
    };
}

#[cfg(feature = "checked_fns")]
impl_legacy_checked! {Rgba => r g b a}
#[cfg(feature = "checked_fns")]
impl_legacy_checked! {Abgr => a b g r}
#[cfg(feature = "checked_fns")]
impl_legacy_checked! {Argb => a r g b}
#[cfg(feature = "checked_fns")]
impl_legacy_checked! {Bgra => b g r a}

#[cfg(feature = "checked_fns")]
impl_legacy_checked! {Bgr => b g r}
#[cfg(feature = "checked_fns")]
impl_legacy_checked! {Rgb => r g b}
#[cfg(feature = "checked_fns")]
impl_legacy_checked! {Grb => r g b}

#[cfg(feature = "checked_fns")]
impl_legacy_checked! {Gray_v08 => 0}
#[cfg(feature = "checked_fns")]
impl_legacy_checked! {GrayAlpha_v08 => 0 1}

#[cfg(test)]
#[allow(deprecated)]
mod test {
    use crate::*;
    use core::num::Wrapping;

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
        use crate::*;

        assert_eq!(
            RGB::new(2, 4, 6),
            RGB::new(1, 2, 3) + RGB { r: 1, g: 2, b: 3 }
        );
        assert_eq!(RGB::new(2., 4., 6.), RGB::new(1., 3., 5.) + 1.);

        assert_eq!(
            RGBA::new_alpha(2f32, 4., 6., 8u32),
            RGBA::new_alpha(1f32, 2., 3., 4u32)
                + RGBA {
                    r: 1f32,
                    g: 2.0,
                    b: 3.0,
                    a: 4u32
                }
        );
        assert_eq!(RGBA::new(2i16, 4, 6, 8), RGBA::new(1, 3, 5, 7) + 1);

        assert_eq!(RGB::new(255, 255, 0), RED_RGB + GREEN_RGB);
        assert_eq!(RGB::new(255, 0, 0), RED_RGB + RGB::new(0, 0, 0));
        assert_eq!(WHITE_RGB, BLACK_RGB + 255);

        assert_eq!(RGBA::new(255, 255, 0, 255), RED_RGBA + GREEN_RGBA);
        assert_eq!(RGBA::new(255, 0, 0, 255), RED_RGBA + RGBA::new(0, 0, 0, 0));
        assert_eq!(WHITE_RGBA, BLACK_RGBA + 255);
    }

    #[test]
    #[cfg(feature = "checked_fns")]
    #[allow(deprecated)]
    fn test_checked_add() {
        assert_eq!(WHITE_RGB.checked_add(WHITE_RGB), None);
        assert_eq!(
            RGB::<u8>::new(255, 255, 255).checked_add(RGB::<u8>::new(255, 0, 0)),
            None
        );
        assert_eq!(
            RGB::<u8>::new(255, 255, 255).checked_add(RGB::<u8>::new(0, 255, 0)),
            None
        );
        assert_eq!(
            RGB::<u8>::new(255, 255, 255).checked_add(RGB::<u8>::new(0, 0, 255)),
            None
        );
        assert_eq!(WHITE_RGBA.checked_add(BLACK_RGBA), Some(WHITE_RGBA));

        assert_eq!(
            RGB::<i8>::new(-128, 2, 3).checked_add(RGB::<i8>::new(-1, 0, 0)),
            None
        );
        assert_eq!(
            RGB::<i8>::new(2, -128, 3).checked_add(RGB::<i8>::new(0, -1, 0)),
            None
        );
        assert_eq!(
            RGB::<i8>::new(2, 2, -128).checked_add(RGB::<i8>::new(0, 0, -1)),
            None
        );
        assert_eq!(
            RGB::<i8>::new(2, 2, -128).checked_add(RGB::<i8>::new(0, 0, 1)),
            Some(RGB::<i8>::new(2, 2, -127))
        );
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn test_add_overflow() {
        assert_ne!(RGBA::new(255u8, 255, 0, 0), RED_RGBA + BLUE_RGBA);
    }

    #[test]
    fn test_sub() {
        assert_eq!(RED_RGB, (WHITE_RGB - GREEN_RGB) - BLUE_RGB);
        assert_eq!(BLACK_RGB, WHITE_RGB - 255);

        assert_eq!(RGBA::new(255, 255, 0, 0), WHITE_RGBA - BLUE_RGBA);
        assert_eq!(BLACK_RGBA, WHITE_RGBA - 255);
    }

    #[test]
    #[cfg(feature = "checked_fns")]
    fn test_checked_sub() {
        assert_eq!(
            RGBA::<u8>::new(2, 4, 6, 111).checked_sub(RGBA::<u8>::new(3, 4, 6, 0)),
            None
        );
        assert_eq!(
            RGB::<u8>::new(2, 4, 6).checked_sub(RGB::<u8>::new(2, 5, 6)),
            None
        );
        assert_eq!(
            RGB::<u8>::new(2, 4, 6).checked_sub(RGB::<u8>::new(2, 4, 7)),
            None
        );
        assert_eq!(
            RGB::<u8>::new(2, 4, 6).checked_sub(RGB::<u8>::new(2, 4, 6)),
            Some(BLACK_RGB)
        );

        assert_eq!(
            RGB::<i8>::new(-128, 4, 6).checked_sub(RGB::<i8>::new(1, 4, 7)),
            None
        );
        assert_eq!(
            RGB::<i8>::new(2, -128, 6).checked_sub(RGB::<i8>::new(2, 1, 7)),
            None
        );
        assert_eq!(
            RGB::<i8>::new(2, 4, -128).checked_sub(RGB::<i8>::new(2, 4, 1)),
            None
        );
        assert_eq!(
            RGB::<i8>::new(2, 4, 6).checked_sub(RGB::<i8>::new(-2, 4, 6)),
            Some(RGB::<i8>::new(4, 0, 0))
        );
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
        assert_eq!(RGB::new(0.5, 1.5, 2.5), RGB::new(1., 3., 5.) * 0.5);
        assert_eq!(RGBA::new(2, 4, 6, 8), RGBA::new(1, 2, 3, 4) * 2);
        assert_eq!(
            RGB::new(0.5, 1.5, 2.5) * RGB::new(1., 3., 5.),
            RGB::new(0.5, 4.5, 12.5)
        );
    }

    #[test]
    fn test_mult_assign() {
        let mut green_rgb = RGB::new(0_u16, 255, 0);
        green_rgb *= 1;
        assert_eq!(RGB::new(0, 255, 0), green_rgb);
        green_rgb *= 2;
        assert_eq!(RGB::new(0, 255 * 2, 0), green_rgb);

        let mut rgb = RGB::new(0.5, 1.5, 2.5);
        rgb *= RGB::new(1., 3., 5.);
        assert_eq!(rgb, RGB::new(0.5, 4.5, 12.5));

        let mut green_rgba = RGBA::new(0_u16, 255, 0, 0);
        green_rgba *= 1;
        assert_eq!(RGBA::new(0, 255, 0, 0), green_rgba);
        green_rgba *= 2;
        assert_eq!(RGBA::new(0, 255 * 2, 0, 0), green_rgba);
    }

    #[test]
    fn sum() {
        use crate::*;

        let s1 = [RGB::new(1_u8, 1, 1), RGB::new(2, 3, 4)]
            .iter()
            .copied()
            .sum::<RGB<u8>>();
        let s2 = [RGB::new(1_u16, 1, 1), RGB::new(2, 3, 4)]
            .iter()
            .copied()
            .sum::<RGB<u16>>();
        let s3 = [
            RGBA::new_alpha(1_u16, 1, 1, Wrapping(1_u16)),
            RGBA::new_alpha(2, 3, 4, Wrapping(5)),
        ]
        .iter()
        .copied()
        .sum::<RGBA<u16, Wrapping<u16>>>();
        let s4 = [
            RGBA::new_alpha(1_u16, 1, 1, 1_u16),
            RGBA::new_alpha(2, 3, 4, 5),
        ]
        .iter()
        .copied()
        .sum::<RGBA<u16, u16>>();
        assert_eq!(s1, RGB::new(3, 4, 5));
        assert_eq!(s2, RGB::new(3, 4, 5));
        assert_eq!(s3, RGBA::new_alpha(3, 4, 5, Wrapping(6)));
        assert_eq!(s4, RGBA::new_alpha(3, 4, 5, 6));
    }
}
