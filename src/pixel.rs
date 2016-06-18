use std;

pub trait ComponentBytes<T> {
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];

    fn as_bytes(&self) -> &[u8] {
        let slice = self.as_slice();
        unsafe {
            std::slice::from_raw_parts(std::mem::transmute(slice.as_ptr()), slice.len() * std::mem::size_of::<T>())
        }
    }
}

pub trait ComponentMap<DestPixel, SrcComponent, DestComponent> {
    fn map<Callback>(&self, mut f: Callback) -> DestPixel
        where Callback: FnMut(SrcComponent) -> DestComponent;
}
