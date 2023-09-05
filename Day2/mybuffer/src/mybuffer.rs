pub struct Buffer<T> {
    buf: Vec<T>,
}

impl<T: std::ops::AddAssign + for<'a> std::iter::Sum<&'a T>> Buffer<T> {
    pub fn new(buf: Vec<T>) -> Buffer<T> {
        Buffer {buf}
    }

    pub fn sum(&self) -> T {
        let res: T = self.buf.iter().sum();
        res
    }
}