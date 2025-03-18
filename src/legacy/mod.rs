pub(crate) mod internal {
    pub mod convert;
    pub mod ops;
    pub mod pixel;
    pub mod rgb;
    pub mod rgba;
}

/// For backwards-compatibility only. This is where BGR/BGRA alernative layouts & grayscale used to be.
///
/// BGR might be useful for some Windows or OpenGL APIs.
pub mod alt;

#[doc(hidden)]
#[deprecated(note = "Renamed to `Rgb`")]
/// Renamed to `Rgb`
pub type RGB<T> = crate::Rgb<T>;
#[doc(hidden)]
#[deprecated(note = "Renamed to `Rgba`")]
/// Renamed to `Rgba`
pub type RGBA<T, A = T> = crate::Rgba<T, A>;

#[test]
#[allow(deprecated)]
fn rgb_works() {
    use crate::*;

    let rgb = Rgb { r: 0_u8, g: 128, b: 255 }.clone();
    assert_eq!(rgb.b, 255);

    assert_eq!(rgb, rgb.iter().map(|ch| ch).collect());

    #[cfg(feature = "as-bytes")]
    {
        assert_eq!(0, [rgb].as_bytes()[0]);
        assert_eq!(128, [rgb].as_bytes()[1]);
        assert_eq!(255, [rgb].as_bytes()[2]);
    }

    let rgb = RGB16 { r: 0_u16, g: 0x7F7F, b: 65535 };
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

    assert_eq!("rgb(1,2,3)", format!("{}", RGB::new(1, 2, 3)));
}

#[test]
#[allow(deprecated)]
fn sub_floats() {
    assert_eq!(RGBA{r:2.5_f64, g:-1.5, b:0., a:5.}, RGBA{r:3.5_f64, g:-0.5, b:-2., a:0.} - RGBA{r:1.0_f64, g:1., b:-2., a:-5.});
}

#[test]
#[allow(deprecated)]
fn into() {
    let a: crate::RGB8 = crate::Rgb { r: 0, g: 1, b: 2 };
    let b: RGB<i16> = a.into();
    let c: RGB<f32> = b.into();
    let d: RGB<f32> = a.into();
    assert_eq!(c, d);
}

#[test]
#[allow(deprecated)]
fn rgba_works() {
    let rgba = RGBA { r: 0_u8, g: 128, b: 255, a: 33 }.clone();
    assert_eq!(rgba.b, 255);
    assert_eq!(rgba.a, 33);

    assert_eq!(rgba, rgba.iter().map(|ch| ch).collect());

    assert_eq!("rgba(1,2,3,4)", format!("{}", RGBA::new(1, 2, 3, 4)));

    assert_eq!(rgba - rgba, RGBA::new(0, 0, 0, 0));
}

#[test]
#[allow(deprecated)]
fn bytes() {
    use crate::*;

    let rgb = RGB8::new(1,2,3);

    #[cfg(feature = "as-bytes")]
    {
        let rgb_arr = [rgb];
        let rgb_bytes = rgb_arr.as_bytes();
        assert_eq!(&[1,2,3], rgb_bytes);
        assert_eq!(rgb_bytes.as_rgba().len(), 0);
        assert_eq!({let t: &[RGBA8] = rgb_bytes.as_pixels(); t}.len(), 0);
        assert_eq!(rgb, rgb_bytes.into_iter().cloned().collect());
        assert_eq!(&[rgb], rgb_bytes.as_rgb());
        assert_eq!(&[rgb], rgb_bytes.as_pixels());
    }
    let mut rgb2 = [rgb];
    assert_eq!(rgb2[..].as_mut_slice().as_rgb_mut(), &mut [rgb]);
    assert_eq!(&mut [rgb], rgb2[..].as_mut_slice().as_pixels_mut());


    #[cfg(feature = "as-bytes")]
    {
        let rgba = RGBA8::new(1,2,3,4);
        let mut rgba_arr = [rgba];
        let rgba_bytes = rgba_arr.as_bytes_mut();
        assert_eq!(&[1,2,3,4], rgba_bytes);
        assert_eq!(&[rgba], rgba_bytes.as_rgba());
        rgba_bytes[3] = 99;
        assert_eq!(RGBA8::new(1,2,3,99), rgba_arr.as_bytes().iter().copied().collect());
    }

    let rgb = RGB16::new(1,2,3);
    let rgb_slice = rgb.as_slice();
    assert_eq!(&[1,2,3], rgb_slice);
    assert_eq!(rgb_slice.as_rgba(), &[]);
    assert_eq!(&[rgb], rgb_slice.as_rgb());
    assert_eq!(rgb, rgb_slice.into_iter().cloned().collect());

    let rgba = RGBA16::new(1,2,3,4);
    let rgba_slice = rgba.as_slice();
    assert_eq!(&[1,2,3,4], rgba_slice);
    assert_eq!(&[1,2,3], rgba_slice.as_rgb()[0].as_slice());
    assert_eq!(&[rgba], rgba_slice.as_rgba());
    assert_eq!(rgba, rgba_slice.iter().copied().collect());
    let mut rgba2 = [rgba];
    assert_eq!(rgba2[..].as_mut_slice().as_rgba_mut(), &mut [rgba]);

    let mut foo = vec![0_u8; 8];
    foo.as_rgba_mut()[1] = RGBA::new(1, 2, 3, 4);
    assert_eq!(&[0_u8, 0, 0, 0, 1, 2, 3, 4], &foo[..]);
}
