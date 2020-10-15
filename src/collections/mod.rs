pub mod ring_buffer;

use crate::elements::Element;

pub trait Buffer: Iterator {
    fn write(&self, el: dyn Element);
}