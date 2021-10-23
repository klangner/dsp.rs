//! Node definition
//! 
//! Node is basic unit of computation
//! 
use anyhow::Result;


pub trait SourceNode<T> {
    fn write_buffer(&mut self, buffer: &mut [T]) -> Result<()>;
}

pub trait SinkNode<T> {
    fn read_buffer(&self, buffer: &[T]) -> Result<()>;
}

pub trait ProcessNode<I, O> {
    fn process_buffer(&self, input_buffer: &[I], output_buffer: &mut [O]) -> Result<()>;
}