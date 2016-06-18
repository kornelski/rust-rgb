use std;
use std::fmt;
use pixel::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RGB<ComponentType> {
    pub r:ComponentType,
    pub g:ComponentType,
    pub b:ComponentType,
}

impl<T: Copy> RGB<T> {
    pub fn new(r: T, g: T, b: T) -> RGB<T> {
        RGB{r:r,g:g,b:b}
    }

}

impl<T: Copy, B> ComponentMap<RGB<B>, T, B> for RGB<T> {
    fn map<F>(&self, mut f: F) -> RGB<B>
        where F: FnMut(T) -> B {
        RGB{
            r:f(self.r),
            g:f(self.g),
            b:f(self.b),
        }
    }
}

impl<T> ComponentBytes<T> for RGB<T> {
    fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(std::mem::transmute(self), 3)
        }
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(std::mem::transmute(self), 3)
        }
    }
}

impl<T: fmt::Display> fmt::Display for RGB<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"rgb({},{},{})", self.r,self.g,self.b)
    }
}

#[test]
fn rgb_test() {
    let neg = RGB::new(1,2,3i32).map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.b, -3);

    let mut px = RGB::new(3,4,5);
    px.as_mut_slice()[1] = 111;
    assert_eq!(111, px.g);

    assert_eq!(RGB{r:1u8,g:2,b:3}, RGB::new(1u8,2,3));
    assert!(RGB{r:1u8,g:1,b:2} < RGB::new(2,1,1));
}
