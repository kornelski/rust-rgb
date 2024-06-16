use crate::*;

/// A pixel which has an alpha component.
pub trait HasAlpha: HetPixel {
    /// Returns the pixels alpha component.
    ///
    /// Use [`HetPixel::try_from_colors_alpha()`] to set the pixels alpha component.
    fn alpha(&self) -> Self::AlphaComponent;
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
        }
    };
}

has_alpha!(Rgba, a);
has_alpha!(Argb, a);
has_alpha!(Bgra, a);
has_alpha!(Abgr, a);
has_alpha!(GrayA, 1);
