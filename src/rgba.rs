use std;
use std::fmt;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RGBA<ComponentType> {
    pub r:ComponentType,
    pub g:ComponentType,
    pub b:ComponentType,
    pub a:ComponentType,
}

impl<T: Copy> RGBA<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> RGBA<T> {
        RGBA{r:r,g:g,b:b,a:a}
    }

    pub fn map<B, F>(&self, mut f: F) -> RGBA<B>
        where F: FnMut(T) -> B {
        RGBA{
            r:f(self.r),
            g:f(self.g),
            b:f(self.b),
            a:f(self.a),
        }
    }
}

impl<T> RGBA<T> {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(std::mem::transmute(self), 4 * std::mem::size_of::<T>())
        }
    }
}

impl<T: fmt::Display> fmt::Display for RGBA<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"rgba({},{},{},{})", self.r,self.g,self.b,self.a)
    }
}

#[test]
fn rgba_map_test() {
    let neg = RGBA::new(1,2,3i32,1000).map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.b, -3);
    assert_eq!(neg.a, -1000);
    assert_eq!(neg, RGBA::from_slice(neg.as_slice()));
    assert!(neg < RGBA::new(0,0,0,0));
}
