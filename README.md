# Stacktools

> **Offer structure representation of data structures** - This crate helps you to deal with data structures such as
> Queue (FiFo), Stack (LiFo) and Priority queue.

[![Build Status](https://travis-ci.org/Freyskeyd/stacktools.svg)](https://travis-ci.org/Freyskeyd/stacktools) [![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]

## Install

Just add it to your `Cargo.toml`:

```toml
[dependencies]
stacktools = "0.1"
```

## Examples

```rust
extern crate stacktools;

fn main() {
    let mut queue: Queue<i32> = stacktools::Queue::new();

    assert!(queue.len() == 0);

    queue.push(1);
    queue.push(2);
    queue.push(3);

    assert!(queue.len() == 3);

    assert_eq!(Some(1), queue.next());
    assert!(queue.len() == 2);

    assert_eq!(Some(2), queue.next());
    assert!(queue.len() == 1);

    assert_eq!(Some(3), queue.next());
    assert!(queue.len() == 0);

    assert_eq!(None, queue.next());
}
```

With PriorityQueue:
```rust
extern crate stacktools;

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

fn main() {
    let mut queue: PriorityQueue<PriorityMessage> = stacktools::PriorityQueue::new();

    assert!(queue.len() == 0);

    queue.push(PriorityMessage {
        priority: Priority::Trivial,
        value: 1,
    });

    queue.push(PriorityMessage {
        priority: Priority::Normal,
        value: 2,
    });

    queue.push(PriorityMessage {
        priority: Priority::Critical,
        value: 3,
    });

    queue.push(PriorityMessage {
        priority: Priority::Critical,
        value: 4,
    });

    assert_eq!(
        Some(PriorityMessage {
            priority: Priority::Critical,
            value: 3,
        }),
        queue.next()
    );

    assert!(queue.len() == 3);

    assert_eq!(
        Some(PriorityMessage {
            priority: Priority::Critical,
            value: 4,
        }),
        queue.next()
    );

    assert!(queue.len() == 2);

    assert_eq!(
        Some(PriorityMessage {
            priority: Priority::Normal,
            value: 2,
        }),
        queue.next()
    );

    assert!(queue.len() == 1);

    assert_eq!(
        Some(PriorityMessage {
            priority: Priority::Trivial,
            value: 1,
        }),
        queue.next()
    );

    assert!(queue.len() == 0);

    assert_eq!(None, queue.next());
}
```
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[Documentation]: https://docs.rs/stacktools
