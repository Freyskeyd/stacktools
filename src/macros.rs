macro_rules! __impl_bounded {
    ($Lhs: ty, $Rhs: ident, $method: ident) => {
        __impl_bounded! { $Lhs, $Rhs, $method, Sized }
    };
    ($Lhs: ty, $Rhs: ident, $method: ident, $Bound: ident) => {
        impl<T: $Bound + ::std::fmt::Debug> BoundedTrait<T> for $Lhs {
            fn bounded(size: usize) -> Self {
                Self {
                    inner: $Rhs::$method(size),
                    capacity: Some(size)
                }
            }

            fn capacity(&self) -> usize {
                match self.capacity {
                    Some(c) => c - self.inner.len(),
                    _ => self.inner.capacity()
                }
            }

            fn max_capacity(&self) -> usize {
                self.capacity() + self.inner.len()
            }

            fn reserve(&mut self, additional: usize) {
                if self.capacity.is_some() {
                    self.inner.reserve(additional);
                    if self.capacity() < additional {
                        self.capacity = Some(self.inner.len() + additional);
                    }
                }
            }
        }
    }
}

macro_rules! __impl_new {
    ($Lhs: ty) => {
        __impl_new! { $Lhs, Sized }
    };
    ($Lhs: ty, $Bound: ident) => {
        impl<T: $Bound> NewTrait for $Lhs {
            fn new() -> Self {
                Self::default()
            }
        }
    }
}

macro_rules! __impl_iterator {
    ($Lhs: ty, $method: ident) => {
        __impl_iterator! { $Lhs, $method, Sized }
    };
    ($Lhs: ty, $method: ident, $Bound: ident) => {
        impl<T: $Bound> Iterator for $Lhs {
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                self.inner.$method()
            }
        }
    }
}

macro_rules! __impl_default {
    ($Lhs: ty, $Rhs: ident) => {
        __impl_default! { $Lhs, $Rhs, Sized }
    };
    ($Lhs: ty, $Rhs: ident, $Bound: ident) => {
        impl<T: $Bound> Default for $Lhs {
            fn default() -> Self {
                Self {
                    inner: $Rhs::new(),
                    capacity: None,
                }
            }
        }
    }
}

macro_rules! __impl_push_trait {
    ($Lhs: ty, $method: ident) => {
        __impl_push_trait! { $Lhs, $method, Sized }
    };
    ($Lhs: ty, $method: ident, $Bound: ident) => {
        impl<T: $Bound> PushTrait<T> for $Lhs {
            fn push(&mut self, value: T) {
                match self.capacity {
                    Some(c) if c > self.inner.len() => self.inner.$method(value),
                    None => self.inner.$method(value),
                    _ => {}
                }
            }
        }
    }
}

macro_rules! __impl_capacity_trait {
    ($Lhs: ty) => {
        __impl_capacity_trait! { $Lhs, Sized }
    };
    ($Lhs: ty, $Bound: ident) => {
        impl<T: $Bound> CapacityTrait<T> for $Lhs {
            fn is_empty(&self) -> bool {
                self.inner.len() == 0
            }

            fn len(&self) -> usize {
                self.inner.len()
            }
        }
    }
}
