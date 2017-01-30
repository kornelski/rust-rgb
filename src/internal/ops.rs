use std::ops::*;
use std::iter::FromIterator;
use super::pixel::*;
use super::rgb::RGB;
use super::rgba::RGBA;

impl<T: Clone + Add> Add for RGB<T>
    where RGB<T>: FromIterator<<T as Add>::Output> {
    type Output = RGB<T>;

    #[inline(always)]
    fn add(self, other: RGB<T>) -> Self::Output {
        self.iter().zip(other.iter()).map(|(l,r)| l+r).collect()
    }
}

impl<T: Clone + Add> Add<RGBA<T>> for RGBA<T>
    where RGBA<T>: FromIterator<<T as Add>::Output>,
        T: From<<T as Add>::Output> {
    type Output = RGBA<T>;

    #[inline(always)]
    fn add(self, other: RGBA<T>) -> Self::Output {
        self.iter().zip(other.iter()).map(|(l,r)| l+r).collect()
    }
}

impl<T: Clone + Sub> Sub for RGB<T>
    where RGB<T>: FromIterator<<T as Sub>::Output> {
    type Output = RGB<T>;

    #[inline(always)]
    fn sub(self, other: RGB<T>) -> Self::Output {
        self.iter().zip(other.iter()).map(|(l,r)| l-r).collect()
    }
}

impl<T: Clone + Sub> Sub<RGBA<T>> for RGBA<T>
    where RGBA<T>: FromIterator<<T as Sub>::Output>,
        T: From<<T as Sub>::Output> {
    type Output = RGBA<T>;

    #[inline(always)]
    fn sub(self, other: RGBA<T>) -> Self::Output {
        self.iter().zip(other.iter()).map(|(l,r)| l-r).collect()
    }
}

impl<T: Clone + Copy + Add> Add<T> for RGB<T>
    where T: Add<Output=T> {
    type Output = RGB<T>;

    #[inline(always)]
    fn add(self, r: T) -> Self::Output {
        self.map(|l|l+r)
    }
}

impl<T: Clone + Copy + Add> Add<T> for RGBA<T>
    where T: Add<Output=T> {
    type Output = RGBA<T>;

    #[inline(always)]
    fn add(self, r: T) -> Self::Output {
        self.map(|l|l+r)
    }
}

impl<T: Clone + Copy + Mul> Mul<T> for RGB<T>
    where T: Mul<Output=T> {
    type Output = RGB<T>;

    #[inline(always)]
    fn mul(self, r: T) -> Self::Output {
        self.map(|l|l*r)
    }
}

impl<T: Clone + Copy + Mul> Mul<T> for RGBA<T>
    where T: Mul<Output=T> {
    type Output = RGBA<T>;

    #[inline(always)]
    fn mul(self, r: T) -> Self::Output {
        self.map(|l|l*r)
    }
}

#[test]
fn test_math() {
    assert_eq!(RGB::new(2,4,6), RGB::new(1,2,3) + RGB{r:1,g:2,b:3});
    assert_eq!(RGB::new(2.,4.,6.), RGB::new(1.,3.,5.) + 1.);
    assert_eq!(RGB::new(0.5,1.5,2.5), RGB::new(1.,3.,5.) * 0.5);

    assert_eq!(RGBA::new(2,4,6,8), RGBA::new(1,2,3,4) + RGBA{r:1,g:2,b:3,a:4});
    assert_eq!(RGBA::new(2i16,4,6,8), RGBA::new(1,3,5,7) + 1);
    assert_eq!(RGBA::new(2,4,6,8), RGBA::new(1,2,3,4) * 2);
}
