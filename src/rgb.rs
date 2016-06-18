use std;
use std::fmt;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RGB<ComponentType> {
    pub r:ComponentType,
    pub g:ComponentType,
    pub b:ComponentType,
}

impl<T: Copy> RGB<T> {
    pub fn new(r: T, g: T, b: T) -> RGB<T> {
        RGB{r:r,g:g,b:b}
    }

    pub fn map<B, F>(&self, mut f: F) -> RGB<B>
        where F: FnMut(T) -> B {
        RGB{
            r:f(self.r),
            g:f(self.g),
            b:f(self.b),
        }
    }
}

impl<T> RGB<T> {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(std::mem::transmute(self), 3 * std::mem::size_of::<T>())
        }
    }
}

impl<T: fmt::Display> fmt::Display for RGB<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"rgb({},{},{})", self.r,self.g,self.b)
    }
}

#[test]
fn rgb_map_test() {
    let neg = RGB::new(1,2,3i32).map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.b, -3);
}
