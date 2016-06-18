use std;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RGBA<T> {
    pub r:T,
    pub g:T,
    pub b:T,
    pub a:T,
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

#[test]
fn rgba_map_test() {
    let neg = RGBA::new(1,2,3i32,1000).map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.b, -3);
    assert_eq!(neg.a, -1000);
}
