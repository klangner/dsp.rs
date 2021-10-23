//! Node definition
//! 
//! Node is basic unit of computation
//! 


pub trait SourceNode<T> {
    fn write_buffer(&mut self, buffer: &mut [T]);
}

pub trait SinkNode<T> {
    fn read_buffer(&self, buffer: &[T]);
}

pub trait ProcessNode<I, O> {
    fn process_buffer(&self, input_buffer: &[I], output_buffer: &mut [O]);
}