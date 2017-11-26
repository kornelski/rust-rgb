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
