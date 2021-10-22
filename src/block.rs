//! Block definition
//! 
//! Block is basic unit of computation
//! 


pub trait SourceBlock<T> {
    fn write_buffer(&mut self, buffer: &mut [T]);
}

pub trait SinkBlock<T> {
    fn read_buffer(&self, buffer: &[T]);
}