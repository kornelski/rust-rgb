use std;
use std::fmt;
use super::pixel::*;
use ::RGB;
use ::RGBA;

impl<T: Clone> RGBA<T> {
    #[must_use] #[inline(always)]
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        RGBA{r:r,g:g,b:b,a:a}
    }

    /// Iterate over all components (length=4)
    #[inline(always)]
    pub fn iter(&self) -> std::iter::Cloned<std::slice::Iter<T>> {
        self.as_slice().iter().cloned()
    }
}

impl<T: Clone, A> RGBA<T,A> {
    /// Copy RGB components out of the RGBA struct
    #[inline(always)]
    pub fn rgb(&self) -> RGB<T> {
        RGB{r:self.r.clone(), g:self.g.clone(), b:self.b.clone()}
    }
}

impl<T, A> RGBA<T,A> {
    /// Provide a mutable view of only RGB components (leaving out alpha). Useful to change color without changing opacity.
    #[inline(always)]
    pub fn rgb_mut(&mut self) -> &mut RGB<T> {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl<T: Copy, A: Clone> RGBA<T, A> {
    /// Create new RGBA with the same alpha value, but different RGB values
    #[inline(always)]
    pub fn map_rgb<F, U, B>(&self, f: F) -> RGBA<U, B>
        where F: FnMut(T) -> U, U: Clone, B: From<A> + Clone {
        self.rgb().map(f).new_alpha(self.a.clone().into())
    }
}

impl<T: Copy, B> ComponentMap<RGBA<B>, T, B> for RGBA<T> {
    #[inline(always)]
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

impl<T> ComponentSlice<T> for RGBA<T> {
    #[inline(always)]
    fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self as *const RGBA<T> as *const T, 4)
        }
    }

    #[inline(always)]
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self as *mut RGBA<T> as *mut T, 4)
        }
    }
}

impl<T> ComponentSlice<T> for [RGBA<T>] {
    #[inline]
    fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.as_ptr() as *const _, self.len() * 4)
        }
    }
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len() * 4)
        }
    }
}

impl<T: Copy> ComponentBytes<T> for [RGBA<T>] {}

impl<T> std::iter::FromIterator<T> for RGBA<T> {
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
        let mut iter = into_iter.into_iter();
        RGBA{r:iter.next().unwrap(), g:iter.next().unwrap(), b:iter.next().unwrap(), a:iter.next().unwrap()}
    }
}

impl<T: fmt::Display, A: fmt::Display> fmt::Display for RGBA<T,A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"rgba({},{},{},{})", self.r,self.g,self.b,self.a)
    }
}

#[test]
fn rgba_test() {
    let neg = RGBA::new(1,2,3i32,1000).map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.rgb().r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.rgb().g, -2);
    assert_eq!(neg.b, -3);
    assert_eq!(neg.rgb().b, -3);
    assert_eq!(neg.a, -1000);
    assert_eq!(neg, neg.as_slice().iter().cloned().collect());
    assert!(neg < RGBA::new(0,0,0,0));

    let neg = RGBA::new(1u8,2,3,4).map_rgb(|c| -(c as i16));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4i16, neg.a);

    let mut px = RGBA{r:1,g:2,b:3,a:4};
    px.as_mut_slice()[3] = 100;
    assert_eq!(1, px.rgb_mut().r);
    assert_eq!(2, px.rgb_mut().g);
    px.rgb_mut().b = 4;
    assert_eq!(4, px.rgb_mut().b);
    assert_eq!(100, px.a);

    let v = vec![RGBA::new(1u8,2,3,4), RGBA::new(5,6,7,8)];
    assert_eq!(&[1,2,3,4,5,6,7,8], v.as_bytes());
}
