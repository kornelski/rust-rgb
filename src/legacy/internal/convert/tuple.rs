use crate::alt::{BGR, BGRA};
use crate::{RGB, RGBA};

impl<T> From<(T, T, T)> for RGB<T> {
    #[inline]
    fn from(other: (T, T, T)) -> Self {
        Self { r: other.0, g: other.1, b: other.2 }
    }
}

impl<T> From<RGB<T>> for (T, T, T) {
    #[inline]
    fn from(value: RGB<T>) -> Self {
        (value.r, value.g, value.b)
    }
}

impl<T, A> From<(T, T, T, A)> for RGBA<T, A> {
    #[inline]
    fn from(other: (T, T, T, A)) -> Self {
        Self {
            r: other.0,
            g: other.1,
            b: other.2,
            a: other.3,
        }
    }
}

impl<T, A> From<RGBA<T, A>> for (T, T, T, A) {
    #[inline]
    fn from(value: RGBA<T, A>) -> Self {
        (value.r, value.g, value.b, value.a)
    }
}

impl<T> From<(T, T, T)> for BGR<T> {
    #[inline(always)]
    fn from(other: (T, T, T)) -> Self {
        Self { b: other.0, g: other.1, r: other.2 }
    }
}

impl<T> From<BGR<T>> for (T, T, T) {
    #[inline(always)]
    fn from(value: BGR<T>) -> Self {
        (value.b, value.g, value.r)
    }
}

impl<T, A> From<(T, T, T, A)> for BGRA<T, A> {
    #[inline(always)]
    fn from(other: (T, T, T, A)) -> Self {
        Self {
            b: other.0,
            g: other.1,
            r: other.2,
            a: other.3,
        }
    }
}

impl<T, A> From<BGRA<T, A>> for (T, T, T, A) {
    #[inline(always)]
    fn from(value: BGRA<T, A>) -> Self {
        (value.b, value.g, value.r, value.a)
    }
}

#[test]
fn converts() {
    assert_eq!((1,2,3), RGB {r:1u8,g:2,b:3}.into());
    assert_eq!(RGB {r:1u8,g:2,b:3}, (1,2,3).into());
    assert_eq!((1,2,3,4), RGBA {r:1,g:2,b:3,a:4}.into());
    assert_eq!(RGBA {r:1u8,g:2,b:3,a:4}, (1,2,3,4).into());
    assert_eq!(BGRA {r:1u8,g:2,b:3,a:4}, (3,2,1,4).into());
    assert_eq!(BGR {r:1u8,g:2,b:3}, (3,2,1).into());
}
