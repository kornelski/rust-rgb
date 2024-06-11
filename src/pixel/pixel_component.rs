use num_traits::{Num, NumAssign, NumCast, NumOps};

/// A trait for all the required super-traits for a pixel component type.
pub trait PixelComponent: Copy + Num + NumCast + NumAssign + NumOps + PartialOrd<Self> {
    /// The minimum component value
    const COMPONENT_MIN: Self;
    /// The maximum component value
    const COMPONENT_MAX: Self;
}

macro_rules! implement_integer {
    ($int:ident) => {
        impl PixelComponent for $int {
            const COMPONENT_MIN: Self = $int::MIN;
            const COMPONENT_MAX: Self = $int::MAX;
        }
    };
}
macro_rules! implement_float {
    ($int:ident) => {
        impl PixelComponent for $int {
            const COMPONENT_MIN: Self = 0.0;
            const COMPONENT_MAX: Self = 1.0;
        }
    };
}
implement_integer!(u8);
implement_integer!(u16);
implement_integer!(u32);
implement_integer!(u64);
implement_integer!(u128);
implement_integer!(i8);
implement_integer!(i16);
implement_integer!(i32);
implement_integer!(i64);
implement_integer!(i128);
implement_integer!(usize);
implement_integer!(isize);
implement_float!(f32);
implement_float!(f64);
