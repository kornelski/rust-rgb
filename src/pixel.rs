/// A Pixel made up of a compile-time known number of contiguously stored `T`s.
///
/// Usually `T` is a small copiable intrinsic type such as `u8`, `u16` or `f32`.
pub trait HomogeneousPixel<T> {
    /// The same pixel type as Self but with a different generic component type.
    type SelfType<U>;
    /// The array form of Self
    type ArrayForm<R>;

    /// Converts an owned `Pixel` type to an array of its components.
    fn components(&self) -> Self::ArrayForm<T>;
    /// Converts an array of components to a `Pixel`.
    fn from_components(components: Self::ArrayForm<T>) -> Self;

    /// Map the pixel to the same outer pixel type with an optionally different inner type.
    fn map<U>(&self, f: impl FnMut(T) -> U) -> Self::SelfType<U>
    where
        U: Copy;
}

/// A pixel with possibly differently typed color and alpha components.
pub trait HeterogeneousPixel<T, A> {
    /// The same pixel type as Self but with a different generic component types.
    type SelfType<U, B>;
    /// The color array form of Self
    type ColorArrayForm<R>;

    /// The color components
    fn colors(&self) -> Self::ColorArrayForm<T>;
    /// The alpha component
    fn alpha(&self) -> A;

    /// Create a new instance given the color and alpha components.
    fn from_colors_alpha(colors: Self::ColorArrayForm<T>, alpha: A) -> Self;

    /// Map the pixel to the same outer pixel type with an optionally different inner color component
    /// type and the exact same alpha component.
    fn map_colors<U>(&self, f: impl FnMut(T) -> U) -> Self::SelfType<U, A>
    where
        U: Copy;

    /// Map the pixel to the same outer pixel type with an optionally different inner color component
    /// type and the exact same alpha component.
    fn map_alpha<B>(&self, f: impl FnMut(A) -> B) -> Self::SelfType<T, B>
    where
        B: Copy;
}

macro_rules! implement_homogeneous_pixel {
    ($name:ident, $length:literal, [$($bit:ident),*]) => {
        impl<T> HomogeneousPixel<T> for $name<T>
        where
            T: Copy,
        {
            type SelfType<U> = $name<U>;
            type ArrayForm<R> = [R; $length];

            fn components(&self) -> Self::ArrayForm<T> {
                [$(self.$bit),*]
            }

            fn from_components(components: Self::ArrayForm<T>) -> Self {
                let mut iter = components.into_iter();

                Self {$($bit: iter.next().unwrap()),*}
            }

            fn map<U>(&self, f: impl FnMut(T) -> U) -> Self::SelfType<U>
            where
                U: Copy,
            {
                Self::SelfType::from_components(self.components().map(f))
            }
        }
    }
}
macro_rules! implement_heterogeneous_pixel {
    ($name:tt, $color_len:literal, [$($color_bit:ident),*], $alpha_bit:ident) => {
        impl<T, A> HeterogeneousPixel<T, A> for $name<T, A>
        where
            T: Copy,
            A: Copy,
        {
            type SelfType<U, B> = $name<U, B>;
            type ColorArrayForm<R> = [R; $color_len];

            fn colors(&self) -> Self::ColorArrayForm<T> {
                [$(self.$color_bit),*]
            }

            fn alpha(&self) -> A {
                self.a
            }

            fn from_colors_alpha(colors: Self::ColorArrayForm<T>, alpha: A) -> Self {
                let mut colors = colors.into_iter();

                Self {
                    $($color_bit: colors.next().unwrap(),)*
                    $alpha_bit: alpha,
                }
            }

            fn map_colors<U>(&self, f: impl FnMut(T) -> U) -> Self::SelfType<U, A>
            where
                U: Copy,
            {
                Self::SelfType::from_colors_alpha(self.colors().map(f), self.alpha())
            }
            fn map_alpha<B>(&self, mut f: impl FnMut(A) -> B) -> Self::SelfType<T, B>
            where
                B: Copy,
            {
                Self::SelfType::from_colors_alpha(self.colors(), f(self.alpha()))
            }
        }
    };
}

mod rgba {
    use crate::*;
    implement_homogeneous_pixel!(Rgba, 4, [r, g, b, a]);
    implement_heterogeneous_pixel!(Rgba, 3, [r, g, b], a);
}
mod abgr {
    use crate::*;
    implement_homogeneous_pixel!(Abgr, 4, [a, b, g, r]);
    implement_heterogeneous_pixel!(Abgr, 3, [b, g, r], a);
}
mod argb {
    use crate::*;
    implement_homogeneous_pixel!(Argb, 4, [a, r, g, b]);
    implement_heterogeneous_pixel!(Argb, 3, [r, g, b], a);
}
mod bgra {
    use crate::*;
    implement_homogeneous_pixel!(Bgra, 4, [b, g, r, a]);
    implement_heterogeneous_pixel!(Bgra, 3, [b, g, r], a);
}
mod gray_alpha {
    use crate::*;
    implement_homogeneous_pixel!(GrayAlpha, 2, [gray, a]);
    implement_heterogeneous_pixel!(GrayAlpha, 1, [gray], a);
}

mod gray {
    use crate::*;
    implement_homogeneous_pixel!(Gray, 1, [gray]);
}
mod bgr {
    use crate::*;
    implement_homogeneous_pixel!(Bgr, 3, [b, g, r]);
}
mod grb {
    use crate::*;
    implement_homogeneous_pixel!(Grb, 3, [g, r, b]);
}
mod rgb {
    use crate::*;
    implement_homogeneous_pixel!(Rgb, 3, [r, g, b]);
}
