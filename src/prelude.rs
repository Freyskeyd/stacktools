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

pub trait BoundedTrait<T>: PushTrait<T> + CapacityTrait<T> {
    fn bounded(size: usize) -> Self;
    fn capacity(&self) -> usize;
    fn reserve(&mut self, additional: usize);
    fn max_capacity(&self) -> usize;
}
