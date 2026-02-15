#![allow(deprecated)]
use rgb::GrayAlpha;
use rgb::Gray;
use rgb::Bgra;
use rgb::Bgr;
#[cfg(feature = "as-bytes")]
use rgb::ComponentBytes;
use rgb::alt::{ABGR, ARGB, BGR, BGRA};
use rgb::{AsPixels, ComponentMap, ComponentSlice, FromSlice, RGB, RGB16, RGB8, RGBA, RGBA16, RGBA8, Rgb, Rgba};
use rgb::prelude::*;

#[test]
fn rgb_works() {
    let rgb = RGB{r:0u8,g:128,b:255}.clone();
    assert_eq!(rgb.b, 255);

    assert_eq!(rgb, rgb.iter().map(|ch| ch).collect());
    assert_eq!(rgb, rgb.iter().collect());

    #[cfg(feature = "as-bytes")]
    {
        assert_eq!(0, [rgb].as_bytes()[0]);
        assert_eq!(128, [rgb].as_bytes()[1]);
        assert_eq!(255, [rgb].as_bytes()[2]);
    }

    let rgb = RGB16{r:0u16,g:0x7F7F,b:65535};
    assert_eq!(rgb.b, 65535);
    assert_eq!(rgb.as_slice()[1], 0x7F7F);

    #[cfg(feature = "as-bytes")]
    {
        assert_eq!(0, [rgb].as_bytes()[0]);
        assert_eq!(0, [rgb].as_bytes()[1]);
        assert_eq!(0x7F, [rgb].as_bytes()[2]);
        assert_eq!(0x7F, [rgb].as_bytes()[3]);
        assert_eq!(0xFF, [rgb].as_bytes()[4]);
        assert_eq!(0xFF, [rgb].as_bytes()[5]);
    }

    assert_eq!("rgb(1,2,3)", format!("{}", RGB::new(1,2,3)));
}

#[test]
fn sub_floats() {
    assert_eq!(RGBA{r:2.5_f64, g:-1.5, b:0., a:5.}, RGBA{r:3.5_f64, g:-0.5, b:-2., a:0.} - RGBA{r:1.0_f64, g:1., b:-2., a:-5.});
}

#[test]
fn into() {
    let a:RGB8 = RGB{r:0,g:1,b:2};
    let b:RGB<i16> = a.into();
    let c:RGB<f32> = b.into();
    let d:RGB<f32> = a.into();
    assert_eq!(c, d);
}

#[test]
fn rgba_works() {
    let rgba = RGBA{r:0u8,g:128,b:255,a:33}.clone();
    assert_eq!(rgba.b, 255);
    assert_eq!(rgba.a, 33);

    assert_eq!(rgba, rgba.iter().map(|ch| ch).collect());
    assert_eq!(rgba, rgba.iter().collect());

    assert_eq!("rgba(1,2,3,4)", format!("{}", RGBA::new(1,2,3,4)));

    assert_eq!(rgba - rgba, RGBA::new(0,0,0,0));
}

#[test]
fn bytes() {
    let rgb = RGB8::new(1,2,3);

    #[cfg(feature = "as-bytes")]
    {
        let rgb_arr = [rgb];
        let rgb_bytes = rgb_arr.as_bytes();
        assert_eq!(&[1,2,3], rgb_bytes);
        assert_eq!(rgb_bytes.as_rgba().len(), 0);
        assert_eq!({let t: &[RGBA8] = rgb_bytes.as_pixels(); t}.len(), 0);
        assert_eq!(rgb, rgb_bytes.into_iter().cloned().collect());
        assert_eq!(rgb, rgb_bytes.iter().copied().collect());
        assert_eq!(&[rgb], rgb_bytes.as_rgb());
        assert_eq!(&[rgb], rgb_bytes.as_pixels());
    }
    let mut rgb2 = [rgb];
    assert_eq!(<_>::as_mut_slice(&mut rgb2[..]).as_rgb_mut(), &mut [rgb]);
    assert_eq!(&mut [rgb], <_>::as_mut_slice(&mut rgb2[..]).as_pixels_mut());


    #[cfg(feature = "as-bytes")]
    {
        let rgba = RGBA8::new(1,2,3,4);
        let mut rgba_arr = [rgba];
        let rgba_bytes = rgba_arr.as_bytes_mut();
        assert_eq!(&[1,2,3,4], rgba_bytes);
        assert_eq!(&[rgba], rgba_bytes.as_rgba());
        rgba_bytes[3] = 99;
        assert_eq!(RGBA8::new(1,2,3,99), rgba_arr.as_bytes().into_iter().cloned().collect());
        assert_eq!(RGBA8::new(1,2,3,99), rgba_arr.as_bytes().iter().copied().collect());
    }

    let rgb = RGB16::new(1,2,3);
    let rgb_slice = rgb.as_slice();
    assert_eq!(&[1,2,3], rgb_slice);
    assert_eq!(rgb_slice.as_rgba(), &[]);
    assert_eq!(&[rgb], rgb_slice.as_rgb());
    assert_eq!(rgb, rgb_slice.into_iter().cloned().collect());
    assert_eq!(rgb, rgb_slice.iter().copied().collect());

    let rgba = RGBA16::new(1,2,3,4);
    let rgba_slice = rgba.as_slice();
    assert_eq!(&[1,2,3,4], rgba_slice);
    assert_eq!(&[1,2,3], rgba_slice.as_rgb()[0].as_slice());
    assert_eq!(&[rgba], rgba_slice.as_rgba());
    assert_eq!(rgba, rgba_slice.iter().copied().collect());
    let mut rgba2 = [rgba];
    assert_eq!(<_>::as_mut_slice(&mut rgba2[..]).as_rgba_mut(), &mut [rgba]);

    let mut foo = vec![0u8; 8];
    foo.as_rgba_mut()[1] = RGBA::new(1,2,3,4);
    assert_eq!(&[0u8,0,0,0,1,2,3,4], &foo[..]);
}

#[test]
#[cfg(feature = "as-bytes")]
#[allow(dead_code)]
fn shared_impl() {
    struct SharedPixelBuffer<Pixel> {
        data: [Pixel; 1],
    }

    impl<Pixel: Clone + rgb::Pod> SharedPixelBuffer<Pixel>
    where
        [Pixel]: rgb::ComponentBytes<u8>,
    {
        pub fn as_bytes(&self) -> &[u8] {
            self.data.as_slice().as_bytes()
        }
    }

    let b = SharedPixelBuffer {
        data: [rgb::RGB8::new(0,0,0)],
    };
    let _ = b.as_bytes();
}


#[test]
#[allow(deprecated)]
fn gray() {
    use rgb::alt::*;

    let rgb: rgb::RGB<_> = Gray(1).into();
    assert_eq!(rgb.r, 1);
    assert_eq!(rgb.g, 1);
    assert_eq!(rgb.b, 1);

    let rgba: rgb::RGBA<_> = Gray(1u8).into();
    assert_eq!(rgba.r, 1);
    assert_eq!(rgba.g, 1);
    assert_eq!(rgba.b, 1);
    assert_eq!(rgba.a, 255);

    let g: GRAY8 = 200.into();
    let g = g.map(|c| c / 2);
    assert_eq!(110, *g + 10);
    assert_eq!(110, 10 + Gray(100).as_ref());

    let ga: GRAYA8 = GrayAlpha(1, 2);
    assert_eq!(ga.gray(), Gray::new(1));
    let mut g2 = ga.clone();
    *g2.gray_mut() = Gray(3);
    assert_eq!(g2.map_gray(|g| g + 1), GRAYA8::new(4, 2));
    assert_eq!(g2.map(|g| g + 1), GrayAlpha(4, 3));
    assert_eq!(g2.0, 3);
    assert_eq!(g2.as_slice(), &[3, 2]);
    assert_eq!(g2.as_mut_slice(), &[3, 2]);
    assert_eq!(g2.with_alpha(13), GrayAlpha(3, 13));
    assert_eq!(g2.map_alpha(|x| x + 3), GrayAlpha(3, 5));

    assert_eq!(<_>::as_slice(&[Gray(1u16), Gray(2)][..]), &[1, 2]);
    assert_eq!(<_>::as_slice(&[GrayAlpha(1u16, 2), GrayAlpha(3, 4)][..]), &[1, 2, 3, 4]);

    let rgba: rgb::RGBA<_> = ga.into();
    assert_eq!(rgba.r, 1);
    assert_eq!(rgba.g, 1);
    assert_eq!(rgba.b, 1);
    assert_eq!(rgba.a, 2);

    let ga: GRAYA16 = GrayAlpha(1, 2);
    let rgba: rgb::RGBA<u16, u16> = ga.into();
    assert_eq!(rgba.r, 1);
    assert_eq!(rgba.g, 1);
    assert_eq!(rgba.b, 1);
    assert_eq!(rgba.a, 2);
}

mod ops {
    use super::*;
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
        assert_eq!(RGB::new(2,4,6), RGB::new(1,2,3) + RGB{r:1,g:2,b:3});
        assert_eq!(RGB::new(2.,4.,6.), RGB::new(1.,3.,5.) + 1.);

        assert_eq!(RGBA::new_alpha(2f32,4.,6.,8u32), RGBA::new_alpha(1f32,2.,3.,4u32) + RGBA{r:1f32,g:2.0,b:3.0,a:4u32});
        assert_eq!(RGBA::new(2i16,4,6,8), RGBA::new(1,3,5,7) + 1);

        assert_eq!(RGB::new(255, 255, 0), RED_RGB+GREEN_RGB);
        assert_eq!(RGB::new(255, 0, 0), RED_RGB+RGB::new(0, 0, 0));
        assert_eq!(WHITE_RGB, BLACK_RGB + 255);

        assert_eq!(RGBA::new(255, 255, 0, 255), RED_RGBA+GREEN_RGBA);
        assert_eq!(RGBA::new(255, 0, 0, 255), RED_RGBA+RGBA::new(0, 0, 0, 0));
        assert_eq!(WHITE_RGBA, BLACK_RGBA + 255);
    }

    #[test]
    #[cfg(feature = "checked_fns")]
    fn test_checked_add() {
        assert_eq!(WHITE_RGB.checked_add(WHITE_RGB), None);
        assert_eq!(RGB::<u8>::new(255, 255, 255).checked_add(RGB::<u8>::new(255, 0, 0)), None);
        assert_eq!(RGB::<u8>::new(255, 255, 255).checked_add(RGB::<u8>::new(0, 255, 0)), None);
        assert_eq!(RGB::<u8>::new(255, 255, 255).checked_add(RGB::<u8>::new(0, 0, 255)), None);
        assert_eq!(WHITE_RGBA.checked_add(BLACK_RGBA), Some(WHITE_RGBA));

        assert_eq!(RGB::<i8>::new(-128, 2, 3).checked_add(RGB::<i8>::new(-1, 0, 0)), None);
        assert_eq!(RGB::<i8>::new(2, -128, 3).checked_add(RGB::<i8>::new(0, -1, 0)), None);
        assert_eq!(RGB::<i8>::new(2, 2, -128).checked_add(RGB::<i8>::new(0, 0, -1)), None);
        assert_eq!(RGB::<i8>::new(2, 2, -128).checked_add(RGB::<i8>::new(0, 0, 1)), Some(RGB::<i8>::new(2, 2, -127)));
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
        assert_eq!(RGBA::<u8>::new(2,4,6,111).checked_sub(RGBA::<u8>::new(3,4,6,0)), None);
        assert_eq!(RGB::<u8>::new(2,4,6).checked_sub(RGB::<u8>::new(2,5,6)), None);
        assert_eq!(RGB::<u8>::new(2,4,6).checked_sub(RGB::<u8>::new(2,4,7)), None);
        assert_eq!(RGB::<u8>::new(2,4,6).checked_sub(RGB::<u8>::new(2,4,6)), Some(BLACK_RGB));

        assert_eq!(RGB::<i8>::new(-128,4,6).checked_sub(RGB::<i8>::new(1,4,7)), None);
        assert_eq!(RGB::<i8>::new(2,-128,6).checked_sub(RGB::<i8>::new(2,1,7)), None);
        assert_eq!(RGB::<i8>::new(2,4,-128).checked_sub(RGB::<i8>::new(2,4,1)), None);
        assert_eq!(RGB::<i8>::new(2,4,6).checked_sub(RGB::<i8>::new(-2,4,6)), Some(RGB::<i8>::new(4,0,0)));
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
        assert_eq!(RGB::new(0.5,1.5,2.5) * RGB::new(1.,3.,5.),
        RGB::new(0.5,4.5,12.5));
    }

    #[test]
    fn test_mult_assign() {
        let mut green_rgb = RGB::new(0u16, 255, 0);
        green_rgb *= 1;
        assert_eq!(RGB::new(0, 255, 0), green_rgb);
        green_rgb *= 2;
        assert_eq!(RGB::new(0, 255*2, 0), green_rgb);

        let mut rgb = RGB::new(0.5,1.5,2.5);
        rgb *= RGB::new(1.,3.,5.);
        assert_eq!(rgb, RGB::new(0.5,4.5,12.5));

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
        let s3 = [RGBA::new_alpha(1u16,1,1,Wrapping(1u16)), RGBA::new_alpha(2,3,4,Wrapping(5))].iter().copied().sum::<RGBA<u16, Wrapping<u16>>>();
        let s4 = [RGBA::new_alpha(1u16,1,1,1u16), RGBA::new_alpha(2,3,4,5)].iter().copied().sum::<RGBA<u16, u16>>();
        assert_eq!(s1, RGB::new(3, 4, 5));
        assert_eq!(s2, RGB::new(3, 4, 5));
        assert_eq!(s3, RGBA::new_alpha(3, 4, 5, Wrapping(6)));
        assert_eq!(s4, RGBA::new_alpha(3, 4, 5, 6));
    }
}


#[cfg(test)]
mod rgb_test {
    use rgb::alt::GRB;
    use super::*;

    #[test]
    fn grb_test() {
        let grb = GRB {g:1,r:2,b:3}.map(|c| c * 2) + 1;
        let rgb: rgb::RGB8 = grb.into();
        assert_eq!(rgb, RGB::new(5,3,7));
    }

    #[test]
    fn sanity_check() {
        let neg = RGB::new(1,2,3i32).map(|x| -x);
        assert_eq!(neg.r, -1);
        assert_eq!(neg.g, -2);
        assert_eq!(neg.b, -3);

        let mut px = RGB::new(3,4,5);
        px.as_mut_slice()[1] = 111;
        assert_eq!(111, px.g);

        assert_eq!(RGBA::new(250,251,252,253), RGB::new(250,251,252).with_alpha(253));

        assert_eq!(RGB{r:1u8,g:2,b:3}, RGB::new(1u8,2,3));
        assert!(RGB{r:1u8,g:1,b:2} < RGB::new(2,1,1));

        let mut h = std::collections::HashSet::new();
        h.insert(px);
        assert!(h.contains(&RGB::new(3,111,5)));
        assert!(!h.contains(&RGB::new(111,5,3)));


        #[cfg(feature = "as-bytes")]
        {
            let v = vec![RGB::new(1u8,2,3), RGB::new(4,5,6)];
            assert_eq!(&[1,2,3,4,5,6], v.as_bytes());
        }

        assert_eq!(RGB::new(0u8,0,0), Default::default());
    }

    #[test]
    #[allow(deprecated)]
    fn test_fmt() {
        let red_rgb = RGB::new(255u8, 0, 0);
        let red_bgr = BGR::new(255u8, 0, 0);
        assert_eq!("#FF0000", &format!("{red_rgb:X}"));
        assert_eq!("#FF0000", &format!("{red_bgr:X}"));

        assert_eq!("#ff0000", &format!("{red_rgb:x}"));
        assert_eq!("#ff0000", &format!("{red_bgr:x}"));

        assert_eq!("rgb(255,0,0)", &format!("{red_rgb}"));
        assert_eq!("bgr(0,0,255)", &format!("{red_bgr}"));
    }
}

#[test]
fn rgba_test() {
    let neg = RGBA::new(1,2,3i32,1000).map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.rgb().r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.rgb().g, -2);
    assert_eq!(neg.b, -3);
    assert_eq!(neg.rgb().b, -3);
    assert_eq!(neg.a, -1000);
    assert_eq!(neg.map_alpha(|x| x+1).a, -999);
    assert_eq!(neg, neg.as_slice().iter().copied().collect());
    assert!(neg < RGBA::new(0,0,0,0));

    #[allow(deprecated)]
    let neg = RGBA::new(1u8,2,3,4).map_rgb(|c| -i16::from(c));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4i16, neg.a);
    let neg = RGBA::new(1u8,2,3,4).map_colors(|c| -(c as i16));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4u8, neg.a);

    let mut px = RGBA{r:1,g:2,b:3,a:4};
    px.as_mut_slice()[3] = 100;
    assert_eq!(1, px.rgb_mut().r);
    assert_eq!(2, px.rgb_mut().g);
    px.rgb_mut().b = 4;
    assert_eq!(4, px.rgb_mut().b);
    assert_eq!(100, px.a);

    #[cfg(feature = "as-bytes")]
    {
        let v = vec![RGBA::new(1u8,2,3,4), RGBA::new(5,6,7,8)];
        assert_eq!(&[1,2,3,4,5,6,7,8], v.as_bytes());
    }
}

#[test]
#[cfg(feature = "as-bytes")]
fn abgr_test() {
    let abgr = ABGR {r:1,g:2,b:3,a:4};
    assert_eq!(4, abgr.as_slice()[0]);
    use rgb::AsPixels;
    assert_eq!(abgr, [abgr].as_bytes().as_pixels()[0]);
}

#[test]
#[allow(deprecated)]
fn bgra_test() {
    let neg = BGRA::new(1, 2, 3i32, 1000).map(|x| -x);
    let _ = neg.as_slice();

    #[cfg(feature = "as-bytes")]
    {
        let _ = [neg].as_bytes();
    }
    assert_eq!(neg.r, -1);
    assert_eq!(neg.bgr().r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.bgr().g, -2);
    assert_eq!(neg.b, -3);
    assert_eq!(neg.bgr().b, -3);
    assert_eq!(neg.a, -1000);
    assert_eq!(&[-3,-2,-1,-1000], neg.as_slice());
    assert!(neg < BGRA::new(0, 0, 0, 0));

    let neg = BGRA::new(1u8, 2u8, 3u8, 4u8).map_rgb(|c| -i16::from(c));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4i16, neg.a);
    #[allow(deprecated)]
    let neg = BGRA::new(1u8, 2u8, 3u8, 4u8).map_colors(|c| -(c as i16));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4u8, neg.a);

    let mut px = BGRA{r:1,g:2,b:3,a:-9}.alpha(4);
    px.as_mut_slice()[3] = 100;
    assert_eq!(1, px.bgr_mut().r);
    assert_eq!(2, px.bgr_mut().g);
    px.bgr_mut().b = 4;
    assert_eq!(4, px.bgr_mut().b);
    assert_eq!(100, px.a);


    #[cfg(feature = "as-bytes")]
    {
        let v = vec![BGRA::new(3u8, 2, 1, 4), BGRA::new(7, 6, 5, 8)];
        assert_eq!(&[1,2,3,4,5,6,7,8], v.as_bytes());
    }
}

#[test]
#[allow(deprecated)]
fn convert_array() {
    use rgb::alt::{BGR8, BGRA8};
    use rgb::RGB8;

    assert_eq!(RGB8::from([1, 2, 3]), RGB8::new(1, 2, 3));
    assert_eq!(Into::<[u8; 3]>::into(RGB8::new(1, 2, 3)), [1, 2, 3]);
    assert_eq!(RGBA8::from([1, 2, 3, 4]), RGBA8::new(1, 2, 3, 4));
    assert_eq!(Into::<[u8; 4]>::into(RGBA8::new(1, 2, 3, 4)), [1, 2, 3, 4]);
    assert_eq!(BGR8::from([3, 2, 1]), BGR8::new(1, 2, 3));
    assert_eq!(Into::<[u8; 3]>::into(BGR8::new(1, 2, 3)), [3, 2, 1]);
    assert_eq!(BGRA8::from([3, 2, 1, 4]), BGRA8::new(1, 2, 3, 4));
    assert_eq!(Into::<[u8; 4]>::into(BGRA8::new(1, 2, 3, 4)), [3, 2, 1, 4]);
}

#[test]
fn argb_converts() {
    let argb = ARGB { a: 0xffu8, r: 0xfa, g: 0xfb, b: 0xfc };
    let rgba = RGBA { a: 0xffu8, r: 0xfa, g: 0xfb, b: 0xfc };

    assert_eq!(RGBA::from(argb), rgba);
    assert_eq!(ARGB::from(rgba), argb);
    assert_eq!(rgba.rgb(), argb.rgb());

    let bgra = BGRA { a: 0xffu8, r: 0x1f, g: 0x2f, b: 0x3f };
    let abgr = ABGR { a: 0xffu8, r: 0x1f, g: 0x2f, b: 0x3f };

    assert_eq!(BGRA::from(abgr), bgra);
    assert_eq!(ABGR::from(bgra), abgr);
}

#[test]
fn converts() {
    assert_eq!([1,2].as_gray(), [Gray::new(1), Gray::new(2)]);
    assert_eq!([3].as_gray_mut(), [Gray::new(3)]);
    assert_eq!([1,2].as_gray_alpha(), [GrayAlpha::new(1, 2)]);
    // excess bytes are ignored
    assert_eq!([1,2,3].as_gray_alpha_mut(), [GrayAlpha::new(1, 2)]);
    assert_eq!([1,2,3,4].as_gray_alpha_mut(), [GrayAlpha::new(1, 2), GrayAlpha::new(3, 4)]);

    assert_eq!(RGBA::new(1u8,2,3,255), RGB::new(1u8,2,3).into());
    assert_eq!(RGBA::new(1u16,2,3,65535), RGB::new(1u16,2,3).into());
    assert_eq!(BGRA{r:1u8,g:2u8,b:3u8,a:255u8}, BGR{r:1u8,g:2u8,b:3u8}.into());
    assert_eq!(BGRA{r:1u8,g:2u8,b:3u8,a:255u8}, RGB{r:1u8,g:2u8,b:3u8}.into());
    assert_eq!(RGBA {r:1u8,g:2,b:3,a:4u8}, BGRA{r:1u8,g:2u8,b:3u8,a:4u8}.into());
    assert_eq!(BGR {r:1u8,g:2,b:3u8}, RGB {r:1u8,g:2,b:3u8}.into());
    assert_eq!(RGB {r:1u16,g:0x5678,b:0xABCDu16}, BGR {r:1u16,g:0x5678,b:0xABCDu16}.into());
    assert_eq!(BGR {r:0x1234567u32,g:2,b:3u32}, RGB {r:0x1234567u32,g:2,b:3u32}.into());

    assert_eq!(&[1u8,2,3,4], RGBA {r:1u8,g:2,b:3,a:4u8}.as_slice());
    assert_eq!(&[1u8,2,3,4], RGBA {r:1u8,g:2,b:3,a:4u8}.as_ref());
    assert_eq!(&[1u8,2,3], RGB {r:1u8,g:2,b:3}.as_slice());
    assert_eq!(&[1u8,2,3], RGB {r:1u8,g:2,b:3}.as_ref());

    assert_eq!(&[1u8,2,3], RGB {r:1u8,g:2,b:3}.as_mut_slice());
    assert_eq!(&[1u8,2,3], RGB {r:1u8,g:2,b:3}.as_mut());
}


#[test]
fn as_refs() {
    let mut r = RGBA::new(1u8,2,3,4u8);
    assert_eq!(&[1,2,3,4], AsRef::<[u8; 4]>::as_ref(&r));
    assert_eq!([1,2,3,4], *AsMut::<[u8; 4]>::as_mut(&mut r));

    let mut r = GrayAlpha::new(1u8,4u8);
    assert_eq!(&[1,4], AsRef::<[u8; 2]>::as_ref(&r));
    assert_eq!([1,4], *AsMut::<[u8; 2]>::as_mut(&mut r));
}

#[test]
fn converts2() {
    assert_eq!((1,2,3), Rgb {r:1u8,g:2,b:3}.into());
    assert_eq!(Rgb {r:1u8,g:2,b:3}, (1,2,3).into());
    assert_eq!((1,2,3,4), Rgba {r:1,g:2,b:3,a:4}.into());
    assert_eq!(Rgba {r:1u8,g:2,b:3,a:4}, (1,2,3,4).into());
    assert_eq!(Bgra {r:1u8,g:2,b:3,a:4}, (3,2,1,4).into());
    assert_eq!(Bgr {r:1u8,g:2,b:3}, (3,2,1).into());
}
