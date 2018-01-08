use core::ops;

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// RGB in reverse byte order
pub struct BGR<ComponentType> {
    /// Blue first
    pub b: ComponentType,
    /// Green
    pub g: ComponentType,
    /// Red last
    pub r: ComponentType,
}

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// BGR+A
pub struct BGRA<ComponentType, AlphaComponentType = ComponentType> {
    /// Blue first
    pub b: ComponentType,
    /// Green
    pub g: ComponentType,
    /// Red
    pub r: ComponentType,
    /// Alpha last
    pub a: AlphaComponentType,
}

pub type BGR8 = BGR<u8>;

/// 16-bit BGR in machine's native endian
pub type BGR16 = BGR<u16>;

pub type BGRA8 = BGRA<u8>;

/// 16-bit BGR in machine's native endian
pub type BGRA16 = BGRA<u16>;

////////////////////////////////////////

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Grayscale. Use `.0` or `*` (deref) to access the value.
pub struct Gray<ComponentType>(
    /// brightness level
    pub ComponentType);

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Grayscale with alpha. Use `.0`/`.1` to access.
pub struct GrayAlpha<ComponentType, AlphaComponentType = ComponentType>(
    /// brightness level
    pub ComponentType,
    /// alpha
    pub AlphaComponentType);

pub type GRAY8 = Gray<u8>;

/// 16-bit gray in machine's native endian
pub type GRAY16 = Gray<u16>;

pub type GRAYA8 = Gray<u8>;

/// 16-bit gray in machine's native endian
pub type GRAYA16 = Gray<u16>;

impl<T> ops::Deref for Gray<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn gray() {
    let rgb: ::RGB<_> = Gray(1).into();
    assert_eq!(rgb.r, 1);
    assert_eq!(rgb.g, 1);
    assert_eq!(rgb.b, 1);

    assert_eq!(110, *Gray(100) + 10);
    assert_eq!(110, 10 + Gray(100).as_ref());

    let rgba: ::RGBA<_> = GrayAlpha(1,2).into();
    assert_eq!(rgba.r, 1);
    assert_eq!(rgba.g, 1);
    assert_eq!(rgba.b, 1);
    assert_eq!(rgba.a, 2);
}
