use std::cmp::Ord;
use std::collections::{BinaryHeap, VecDeque};
use std::collections::binary_heap;
use std::collections::vec_deque;

pub mod prelude;
#[macro_use]
mod macros;

use prelude::*;

#[derive(Clone)]
pub struct Queue<T> {
    inner: VecDeque<T>,
    capacity: Option<usize>,
}

#[derive(Clone)]
pub struct Stack<T> {
    inner: VecDeque<T>,
    capacity: Option<usize>,
}

#[derive(Clone)]
pub struct PriorityQueue<T: Ord> {
    inner: BinaryHeap<T>,
    capacity: Option<usize>,
}

impl<T> Queue<T> {
    pub fn iter(&self) -> vec_deque::Iter<T> {
        self.inner.iter()
    }
}

impl<T> Stack<T> {
    pub fn iter(&self) -> vec_deque::Iter<T> {
        self.inner.iter()
    }
}

impl<T> PriorityQueue<T>
where
    T: Ord,
{
    pub fn iter(&self) -> binary_heap::Iter<T> {
        self.inner.iter()
    }
}


__impl_push_trait! { PriorityQueue<T>, push, Ord }
__impl_push_trait! { Stack<T>, push_front }
__impl_push_trait! { Queue<T>, push_back }

__impl_bounded! { PriorityQueue<T>, BinaryHeap, with_capacity, Ord }
__impl_bounded! { Queue<T>, VecDeque, with_capacity }
__impl_bounded! { Stack<T>, VecDeque, with_capacity }

__impl_capacity_trait! { PriorityQueue<T>, Ord }
__impl_capacity_trait! { Queue<T> }
__impl_capacity_trait! { Stack<T> }

__impl_new! { PriorityQueue<T>, Ord }
__impl_new! { Queue<T> }
__impl_new! { Stack<T> }

__impl_iterator! { Queue<T>, pop_front }
__impl_iterator! { Stack<T>, pop_front }
__impl_iterator! { PriorityQueue<T>, pop, Ord }

__impl_default! { Queue<T>, VecDeque }
__impl_default! { Stack<T>, VecDeque }
__impl_default! { PriorityQueue<T>, BinaryHeap, Ord }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ord;
    use std::cmp::Ordering;

    #[test]
    fn fifo_instantiate() {
        let mut fifo: Queue<i32> = Queue::new();

        assert!(fifo.len() == 0);
        fifo.push(1);
        fifo.push(2);
        fifo.push(3);
        assert!(fifo.len() == 3);

        {
            let mut iter = fifo.iter();

            assert_eq!(Some(&1), iter.next());
            assert_eq!(Some(&2), iter.next());
            assert_eq!(Some(&3), iter.next());
        }

        assert_eq!(Some(1), fifo.next());
        assert!(fifo.len() == 2);

        assert_eq!(Some(2), fifo.next());
        assert!(fifo.len() == 1);

        assert_eq!(Some(3), fifo.next());
        assert!(fifo.len() == 0);

        assert_eq!(None, fifo.next());
    }

    #[test]
    fn lifo_instantiate() {
        let mut lifo: Stack<i32> = Stack::new();

        assert!(lifo.len() == 0);
        lifo.push(1);
        lifo.push(2);
        lifo.push(3);
        assert!(lifo.len() == 3);

        {
            let mut iter = lifo.iter();

            assert_eq!(Some(&3), iter.next());
            assert_eq!(Some(&2), iter.next());
            assert_eq!(Some(&1), iter.next());
        }

        assert_eq!(Some(3), lifo.next());
        assert!(lifo.len() == 2);

        assert_eq!(Some(2), lifo.next());
        assert!(lifo.len() == 1);

        assert_eq!(Some(1), lifo.next());
        assert!(lifo.len() == 0);

        assert_eq!(None, lifo.next());
    }

    #[derive(Copy, Eq, Debug, PartialOrd, PartialEq)]
    enum Priority {
        Trivial = 1,
        Normal,
        Critical,
    }

    impl Clone for Priority {
        #[inline]
        fn clone(&self) -> Priority {
            *self
        }
    }

    impl Ord for Priority {
        #[inline]
        fn cmp(&self, other: &Priority) -> Ordering {
            (*self as usize).cmp(&(*other as usize))
        }
    }

    #[derive(Eq, PartialEq, Debug)]
    struct PriorityMessage {
        priority: Priority,
        value: i32,
    }

    impl Ord for PriorityMessage {
        fn cmp(&self, other: &PriorityMessage) -> Ordering {
            self.priority.cmp(&other.priority)
        }
    }

    impl PartialOrd for PriorityMessage {
        fn partial_cmp(&self, other: &PriorityMessage) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[test]
    fn priority_instantiate() {
        let mut prio: PriorityQueue<PriorityMessage> = PriorityQueue::new();

        assert!(prio.len() == 0);
        prio.push(PriorityMessage {
            priority: Priority::Trivial,
            value: 1,
        });
        prio.push(PriorityMessage {
            priority: Priority::Normal,
            value: 2,
        });
        prio.push(PriorityMessage {
            priority: Priority::Critical,
            value: 3,
        });
        prio.push(PriorityMessage {
            priority: Priority::Critical,
            value: 4,
        });

        assert!(prio.len() == 4);

        {
            let mut iter = prio.iter();

            assert_eq!(
                Some(&PriorityMessage {
                    priority: Priority::Critical,
                    value: 3,
                }),
                iter.next()
            );
            assert_eq!(
                Some(&PriorityMessage {
                    priority: Priority::Critical,
                    value: 4,
                }),
                iter.next()
            );
            assert_eq!(
                Some(&PriorityMessage {
                    priority: Priority::Normal,
                    value: 2,
                }),
                iter.next()
            );
            assert_eq!(
                Some(&PriorityMessage {
                    priority: Priority::Trivial,
                    value: 1,
                }),
                iter.next()
            );
        }

        assert_eq!(
            Some(PriorityMessage {
                priority: Priority::Critical,
                value: 3,
            }),
            prio.next()
        );
        assert!(prio.len() == 3);

        assert_eq!(
            Some(PriorityMessage {
                priority: Priority::Critical,
                value: 4,
            }),
            prio.next()
        );
        assert!(prio.len() == 2);

        assert_eq!(
            Some(PriorityMessage {
                priority: Priority::Normal,
                value: 2,
            }),
            prio.next()
        );
        assert!(prio.len() == 1);

        assert_eq!(
            Some(PriorityMessage {
                priority: Priority::Trivial,
                value: 1,
            }),
            prio.next()
        );
        assert!(prio.len() == 0);

        assert_eq!(None, prio.next());
    }

    macro_rules! execute_circular_test_bounded {
        ($T: ty, $n: expr, $test: ident) => {
            $test(Stack::bounded($n));
            $test(Queue::bounded($n));
            $test(PriorityQueue::bounded($n));
        }
    }

    #[test]
    fn bounded() {
        fn test<X, T: PushTrait<i32> + CapacityTrait<X>>(mut x: T) {
            assert_eq!(x.len(), 0);
            x.push(1);
            x.push(2);
            x.push(3);
            x.push(4);
            x.push(5);
            x.push(6);
            assert_eq!(x.len(), 5);
        };

        execute_circular_test_bounded! {i32, 5, test}
    }

    #[test]
    fn bounded_capacity() {
        fn test<T: BoundedTrait<i32>>(x: T) {
            assert_eq!(x.len(), 0);
            assert_eq!(x.capacity(), 5);
        };

        execute_circular_test_bounded! {i32, 5, test}
    }

    #[test]
    fn bounded_reserve() {
        fn test<T: BoundedTrait<i32>>(mut x: T) {
            assert_eq!(x.len(), 0);
            x.reserve(5);
            assert_eq!(x.capacity(), 5);
            x.push(1);
            assert_eq!(x.capacity(), 4);
            assert_eq!(x.len(), 1);
        };

        execute_circular_test_bounded! {i32, 5, test}
    }

    #[test]
    fn bounded_queue_capacity_hard() {
        let mut queue: Queue<i32> = Queue::bounded(10);
        queue.push(1);
        queue.push(2);

        assert_eq!(queue.len(), 2);
        assert_eq!(queue.capacity(), 8);

        queue.push(3);

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.capacity(), 7);

        let _ = queue.next();

        assert_eq!(queue.len(), 2);
        assert_eq!(queue.capacity(), 8);
        let _ = queue.next();
        let _ = queue.next();

        assert_eq!(queue.len(), 0);
        assert_eq!(queue.capacity(), 10);

        queue.reserve(15);

        assert_eq!(queue.len(), 0);
        assert_eq!(queue.capacity(), 15);

        (0..10).for_each(|x| queue.push(x));

        assert_eq!(queue.len(), 10);
        assert_eq!(queue.capacity(), 5);
        assert_eq!(queue.max_capacity(), 15);

        queue.reserve(10);

        assert_eq!(queue.len(), 10);
        assert_eq!(queue.capacity(), 10);
        assert_eq!(queue.max_capacity(), 20);

        queue.reserve(5);

        assert_eq!(queue.len(), 10);
        assert_eq!(queue.capacity(), 10);
        assert_eq!(queue.max_capacity(), 20);

        (0..20).for_each(|x| queue.push(x));

        assert_eq!(queue.len(), 20);
        assert_eq!(queue.max_capacity(), 20);
    }
}
