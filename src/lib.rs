
mod rgb;
mod rgba;

pub use rgb::*;
pub use rgba::*;

pub type RGB8 = RGB<u8>;
pub type RGB16 = RGB<u16>;
pub type RGBA8 = RGBA<u8>;
pub type RGBA16 = RGBA<u16>;

#[test]
fn rgb_works() {
    let rgb = RGB{r:0u8,g:128,b:255};
    assert_eq!(rgb.b, 255);

    assert_eq!(0, rgb.as_bytes()[0]);
    assert_eq!(128, rgb.as_bytes()[1]);
    assert_eq!(255, rgb.as_bytes()[2]);

    let rgb = RGB16{r:0u16,g:0x7F7F,b:65535};
    assert_eq!(rgb.b, 65535);

    assert_eq!(0, rgb.as_bytes()[0]);
    assert_eq!(0, rgb.as_bytes()[1]);
    assert_eq!(0x7F, rgb.as_bytes()[2]);
    assert_eq!(0x7F, rgb.as_bytes()[3]);
    assert_eq!(0xFF, rgb.as_bytes()[4]);
    assert_eq!(0xFF, rgb.as_bytes()[5]);
}

#[test]
fn rgba_works() {
    let rgba = RGBA{r:0u8,g:128,b:255,a:33};
    assert_eq!(rgba.b, 255);
    assert_eq!(rgba.a, 33);
}
