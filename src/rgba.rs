use std;
use std::fmt;
use pixel::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RGBA<ComponentType> {
    pub r:ComponentType,
    pub g:ComponentType,
    pub b:ComponentType,
    pub a:ComponentType,
}

impl<T: Clone> RGBA<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> RGBA<T> {
        RGBA{r:r,g:g,b:b,a:a}
    }

    pub fn iter(&self) -> std::iter::Cloned<std::slice::Iter<T>> {
        self.as_slice().iter().cloned()
    }
}

impl<T: Copy, B> ComponentMap<RGBA<B>, T, B> for RGBA<T> {
    fn map<F>(&self, mut f: F) -> RGBA<B>
        where F: FnMut(T) -> B {
        RGBA{
            r:f(self.r),
            g:f(self.g),
            b:f(self.b),
            a:f(self.a),
        }
    }
}

impl<T> ComponentBytes<T> for RGBA<T> {
    fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(std::mem::transmute(self), 4)
        }
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(std::mem::transmute(self), 4)
        }
    }
}

impl<T> std::iter::FromIterator<T> for RGBA<T> {
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> RGBA<T> {
        let mut iter = into_iter.into_iter();
        RGBA{r:iter.next().unwrap(), g:iter.next().unwrap(), b:iter.next().unwrap(), a:iter.next().unwrap()}
    }
}

impl<T: fmt::Display> fmt::Display for RGBA<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"rgba({},{},{},{})", self.r,self.g,self.b,self.a)
    }
}

#[test]
fn rgba_test() {
    let neg = RGBA::new(1,2,3i32,1000).map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.b, -3);
    assert_eq!(neg.a, -1000);
    assert_eq!(neg, neg.as_slice().iter().cloned().collect());
    assert!(neg < RGBA::new(0,0,0,0));

    let mut px = RGBA{r:1,g:2,b:3,a:4};
    px.as_mut_slice()[3] = 100;
    assert_eq!(100, px.a);
}
