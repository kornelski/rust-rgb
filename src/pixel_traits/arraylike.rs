use core::borrow::{Borrow, BorrowMut};
use core::ops::{Index, IndexMut};

/// A trait used when returning arrays from the two pixel traits due to the lack of the const
/// generic expressions feature on stable rust.
///
/// See [`HetPixel::color_array()`](crate::HetPixel::color_array) as
/// an example.
pub trait ArrayLike<T>:
    AsRef<[T]>
    + AsMut<[T]>
    + Index<usize, Output = T>
    + IndexMut<usize>
    + Borrow<[T]>
    + BorrowMut<[T]>
    + IntoIterator<Item = T>
{
}
impl<T, const N: usize> ArrayLike<T> for [T; N] {}
