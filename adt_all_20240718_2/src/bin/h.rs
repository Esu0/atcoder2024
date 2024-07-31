use std::collections::HashMap;

use proconio::{input, marker};
use stack::PersistentStackPool;

#[allow(dead_code)]
mod stack {
    use std::{
        alloc::{handle_alloc_error, Layout},
        cell::{Cell, UnsafeCell},
        mem::MaybeUninit,
        ptr::NonNull,
    };

    pub struct PersistentStackPool<T> {
        pool: Box<[UnsafeCell<MaybeUninit<Node<T>>>]>,
        len: Cell<usize>,
    }

    // unsafe impl<T: Send> Send for PersistentStackPool<T> {}

    impl<T> Drop for PersistentStackPool<T> {
        fn drop(&mut self) {
            let len = self.len.get();
            for node in &mut self.pool[..len] {
                unsafe {
                    node.get_mut().assume_init_drop();
                }
            }
        }
    }

    pub struct PersistentStack<'a, T> {
        head: usize,
        pool: &'a PersistentStackPool<T>,
    }

    impl<'a, T> Clone for PersistentStack<'a, T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<'a, T> Copy for PersistentStack<'a, T> {}

    struct Node<T> {
        prev: usize,
        value: T,
    }

    impl<T> Node<T> {
        fn new(prev: usize, value: T) -> Self {
            Self { prev, value }
        }
    }

    impl<T> PersistentStackPool<T> {
        pub fn new(size: usize) -> Self {
            if size == 0 {
                return Self {
                    pool: Box::new([]),
                    len: Cell::new(0),
                };
            }
            unsafe {
                let layout = Layout::array::<MaybeUninit<Node<T>>>(size).unwrap();
                let ptr = std::alloc::alloc(layout) as *mut _;
                let Some(ptr) = NonNull::new(ptr) else {
                    handle_alloc_error(layout);
                };
                let pool = NonNull::slice_from_raw_parts(ptr, size);
                Self {
                    pool: Box::from_raw(pool.as_ptr()),
                    len: Cell::new(0),
                }
            }
        }

        pub fn get_empty_stack(&self) -> PersistentStack<'_, T> {
            PersistentStack {
                head: usize::MAX,
                pool: self,
            }
        }

        #[cfg(test)]
        fn check_invariant(&self) {
            let len = self.len.get();
            assert!(len <= self.pool.len());
            for i in 0..len {
                let node_prev = unsafe { (*self.pool[i].get()).assume_init_ref().prev };
                assert!(node_prev == usize::MAX || node_prev < len);
            }
        }
    }

    impl<'a, T> PersistentStack<'a, T> {
        pub fn push(&self, value: T) -> Self {
            let new_node = Node::new(self.head, value);
            let pool_last = self.pool.len.get();
            unsafe {
                self.pool.pool[pool_last]
                    .get()
                    .write(MaybeUninit::new(new_node));
            }
            self.pool.len.set(pool_last + 1);
            Self {
                head: pool_last,
                pool: self.pool,
            }
        }

        pub fn top(&self) -> Option<&T> {
            if self.head == usize::MAX {
                None
            } else {
                Some(unsafe { &(*self.pool.pool[self.head].get()).assume_init_ref().value })
            }
        }

        pub fn pop(&self) -> Self {
            if self.head == usize::MAX {
                *self
            } else {
                Self {
                    head: unsafe { (*self.pool.pool[self.head].get()).assume_init_ref().prev },
                    pool: self.pool,
                }
            }
        }
    }
}

fn main() {
    input! {
        q: usize,
    }
    let pool = PersistentStackPool::new(q);
    let mut map = HashMap::new();
    let mut a = pool.get_empty_stack();
    for _ in 0..q {
        input! {
            s: marker::Bytes,
        }
        match s[0] {
            b'A' => {
                input! {
                    x: u32,
                }
                a = a.push(x);
            }
            b'D' => {
                a = a.pop();
            }
            b'S' => {
                input! {
                    y: u32,
                }
                map.insert(y, a);
            }
            b'L' => {
                input! {
                    z: u32,
                }
                a = map.get(&z).copied().unwrap_or_else(|| pool.get_empty_stack());
            }
            _ => unreachable!(),
        }
        if let Some(&t) = a.top() {
            print!("{} ", t);
        } else {
            print!("-1 ");
        }
    }
}
