use crate::Abgr;
use crate::Argb;
use crate::Bgra;
use crate::GrayAlpha;
use crate::HetPixel;
use crate::LumaA;
use crate::PixelComponent;
use crate::Rgba;

/// A pixel which has an alpha component.
pub trait HasAlpha: HetPixel {
    /// Returns a copy of the pixel's alpha component.
    fn alpha(&self) -> Self::AlphaComponent;
    /// Returns a mutable borrow of the pixel's alpha component.
    fn alpha_mut(&mut self) -> &mut Self::AlphaComponent;
}

macro_rules! has_alpha {
    ($name:ident, $alpha_bit: tt) => {
        impl<T, A> HasAlpha for $name<T, A>
        where
            T: PixelComponent,
            A: PixelComponent,
        {
            fn alpha(&self) -> Self::AlphaComponent {
                self.$alpha_bit
            }
            fn alpha_mut(&mut self) -> &mut Self::AlphaComponent {
                &mut self.$alpha_bit
            }
        }
    };
}

has_alpha!(Rgba, a);
has_alpha!(Argb, a);
has_alpha!(Bgra, a);
has_alpha!(Abgr, a);
has_alpha!(GrayAlpha, 1);
has_alpha!(LumaA, a);
