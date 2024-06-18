use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Index, IndexMut},
};

use crate::PixelComponent;

/// A trait used when returning arrays from the two pixel traits due to the lack of the const
/// generic expression feature on stable rust.
///
/// A blanket implementation is provided but only for item types which implement
/// [`PixelComponent`].
pub trait ArrayLike<T>:
    Copy
    + AsRef<[T]>
    + AsMut<[T]>
    + Index<usize, Output = T>
    + IndexMut<usize>
    + Borrow<[T]>
    + BorrowMut<[T]>
    + IntoIterator<Item = T>
{
}
impl<T, const N: usize> ArrayLike<T> for [T; N] where T: PixelComponent {}
