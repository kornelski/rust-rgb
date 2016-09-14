use std;
use std::fmt;
use super::pixel::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RGB<ComponentType> {
    pub r:ComponentType,
    pub g:ComponentType,
    pub b:ComponentType,
}

impl<T: Clone> RGB<T> {
    /// Convenience function for creating a new pixel
    pub fn new(r: T, g: T, b: T) -> RGB<T> {
        RGB{r:r,g:g,b:b}
    }

    /// Iterate over color components (R, G, and B)
    pub fn iter(&self) -> std::iter::Cloned<std::slice::Iter<T>> {
        self.as_slice().iter().cloned()
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
            std::slice::from_raw_parts(self as *const RGB<T> as *const T, 3)
        }
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self as *mut RGB<T> as *mut T, 3)
        }
    }
}

impl<T> std::iter::FromIterator<T> for RGB<T> {
    /// Takes exactly 3 elements from the iterator and creates a new instance.
    /// Panics if there are fewer elements in the iterator.
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> RGB<T> {
        let mut iter = into_iter.into_iter();
        RGB{r:iter.next().unwrap(), g:iter.next().unwrap(), b:iter.next().unwrap()}
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

    let mut h = std::collections::HashSet::new();
    h.insert(px);
    assert!(h.contains(&RGB::new(3,111,5)));
    assert!(!h.contains(&RGB::new(111,5,3)));
}
