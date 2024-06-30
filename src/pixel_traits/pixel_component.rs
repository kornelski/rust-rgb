/// A trait for all the required super-traits for a pixel component type.
///
/// Implementations are provided for all intrinsic types.
///
/// Integers such as `u8` set [`PixelComponent::COMPONENT_MIN`]
/// equal to [`u8::MIN`] and [`PixelComponent::COMPONENT_MAX`] to
/// [`u8::MAX`].
///
/// Floats such as `f32` set [`PixelComponent::COMPONENT_MIN`]
/// equal to `0.0` and [`PixelComponent::COMPONENT_MAX`] to
/// `1.0`.
///
/// If you require a different range for one of the intrinstic types,
/// make a newtype such as:
///
/// # Examples
///
/// ```
/// use rgb::PixelComponent;
///
/// #[derive(Clone, Copy)]
/// struct FullRangeF32(f32);
///
/// impl PixelComponent for FullRangeF32 {
///     const COMPONENT_MIN: Self = Self(f32::MIN);
///     const COMPONENT_MAX: Self = Self(f32::MAX);
/// }
/// ```
pub trait PixelComponent: Copy + 'static {
    /// The minimum component value
    const COMPONENT_MIN: Self;
    /// The maximum component value
    const COMPONENT_MAX: Self;
}

macro_rules! integer {
    ($name:ident) => {
        impl PixelComponent for $name {
            const COMPONENT_MIN: Self = $name::MIN;
            const COMPONENT_MAX: Self = $name::MAX;
        }
    };
}
macro_rules! float {
    ($name:ident) => {
        impl PixelComponent for $name {
            const COMPONENT_MIN: Self = 0.0;
            const COMPONENT_MAX: Self = 1.0;
        }
    };
}

impl PixelComponent for bool {
    const COMPONENT_MIN: Self = false;
    const COMPONENT_MAX: Self = true;
}

integer!(u8);
integer!(u16);
integer!(u32);
integer!(u64);
integer!(u128);
integer!(i8);
integer!(i16);
integer!(i32);
integer!(i64);
integer!(i128);
integer!(usize);
integer!(isize);
float!(f32);
float!(f64);
