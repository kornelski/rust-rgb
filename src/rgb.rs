use std;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RGB<T> {
    pub r:T,
    pub g:T,
    pub b:T,
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

#[test]
fn rgb_map_test() {
    let neg = RGB::new(1,2,3i32).map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.b, -3);
}
