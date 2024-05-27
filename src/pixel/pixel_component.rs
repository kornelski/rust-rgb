/// A trait for all the required super-traits for a pixel component type.
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
