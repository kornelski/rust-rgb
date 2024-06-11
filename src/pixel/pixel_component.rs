use num_traits::{Num, NumAssign, NumCast, NumOps};

/// A trait for all the required super-traits for a pixel component type.
pub trait PixelComponent: Copy + Num + NumCast + NumAssign + NumOps + PartialOrd<Self> {
    /// The minimum component value
    const COMPONENT_MIN: Self;
    /// The maximum component value
    const COMPONENT_MAX: Self;
}

macro_rules! integer {
    ($int:ident) => {
        impl PixelComponent for $int {
            const COMPONENT_MIN: Self = $int::MIN;
            const COMPONENT_MAX: Self = $int::MAX;
        }
    };
}
macro_rules! float {
    ($int:ident) => {
        impl PixelComponent for $int {
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
