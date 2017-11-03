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

pub trait UnBoundedTrait<T>: NewTrait + PushTrait<T> + CapacityTrait<T> {
    fn reserve(&mut self, additional: usize);
    fn capacity(&self) -> usize;
}

pub trait BoundedTrait<T>: UnBoundedTrait<T> {
    fn bounded(size: usize) -> Self;
    fn max_capacity(&self) -> usize;
}
