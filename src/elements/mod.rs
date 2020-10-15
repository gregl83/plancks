pub mod event;

pub trait Element {
    fn write(&mut self);
}