use std;
use std::fmt;
use super::pixel::*;
use ::RGB;
use ::RGBA;

impl<T: Clone> RGB<T> {
    /// Convenience function for creating a new pixel
    #[must_use] #[inline(always)]
    pub fn new(r: T, g: T, b: T) -> Self {
        RGB{r:r,g:g,b:b}
    }

    /// Iterate over color components (R, G, and B)
    #[inline(always)]
    pub fn iter(&self) -> std::iter::Cloned<std::slice::Iter<T>> {
        self.as_slice().iter().cloned()
    }

    // Convenience function for converting to RGBA
    #[inline(always)]
    pub fn alpha(&self, a: T) -> RGBA<T> {
        RGBA {
            r:self.r.clone(),
            g:self.g.clone(),
            b:self.b.clone(),
            a:a,
        }
    }

    // Convenience function for converting to RGBA with alpha channel of a different type than type of the pixels
    #[inline(always)]
    pub fn new_alpha<A>(&self, a: A) -> RGBA<T,A> {
        RGBA {
            r:self.r.clone(),
            g:self.g.clone(),
            b:self.b.clone(),
            a:a,
        }
    }
}

impl<T: Copy, B> ComponentMap<RGB<B>, T, B> for RGB<T> {
    #[inline(always)]
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
    #[inline(always)]
    fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self as *const RGB<T> as *const T, 3)
        }
    }

    #[inline(always)]
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self as *mut RGB<T> as *mut T, 3)
        }
    }
}

impl ByteSlice for [RGB<u8>] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.as_ptr() as *const _, self.len() * std::mem::size_of::<RGB<u8>>())
        }
    }
    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len() * std::mem::size_of::<RGB<u8>>())
        }
    }
}

impl<T> std::iter::FromIterator<T> for RGB<T> {
    /// Takes exactly 3 elements from the iterator and creates a new instance.
    /// Panics if there are fewer elements in the iterator.
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
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

    assert_eq!(RGBA::new(250,251,252,253), RGB::new(250,251,252).alpha(253));

    assert_eq!(RGB{r:1u8,g:2,b:3}, RGB::new(1u8,2,3));
    assert!(RGB{r:1u8,g:1,b:2} < RGB::new(2,1,1));

    let mut h = std::collections::HashSet::new();
    h.insert(px);
    assert!(h.contains(&RGB::new(3,111,5)));
    assert!(!h.contains(&RGB::new(111,5,3)));

    let v = vec![RGB::new(1u8,2,3), RGB::new(4,5,6)];
    assert_eq!(&[1,2,3,4,5,6], v.as_bytes());

    assert_eq!(RGB::new(0u8,0,0), Default::default());
}
