pub trait CapacityTrait<T> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

pub trait PushTrait<T> {
    fn push(&mut self, value: T);
}

pub trait NewTrait {
    fn new() -> Self;
}
