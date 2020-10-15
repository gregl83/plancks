pub mod ring_buffer;

use crate::elements::Element;

pub trait StreamingIterator<'a> {
    type Item;
    fn next(&'a mut self) -> Option<&'a mut Self::Item>;
}

pub trait Buffer<'a, E: Element>: StreamingIterator<'a> {
    fn write(&self, el: E);
}