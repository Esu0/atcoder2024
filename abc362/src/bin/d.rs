use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

use std::num::NonZeroUsize;

use node::{NodeArray, NodeRef};

mod node {
    use std::num::NonZeroUsize;

#[derive(Debug)]
pub struct NodeArray<K, T> {
    nodes: Vec<Option<Node<K, T>>>,
}

impl<K, T> NodeArray<K, T> {
    pub const fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn get(&self, index: NodeRef) -> &Node<K, T> {
        unsafe {
            self.nodes
                .get_unchecked(index.0.get())
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn get_mut(&mut self, index: NodeRef) -> &mut Node<K, T> {
        unsafe {
            self.nodes
                .get_unchecked_mut(index.0.get())
                .as_mut()
                .unwrap_unchecked()
        }
    }

    pub unsafe fn destroy(&mut self, index: NodeRef) -> Node<K, T> {
        self.nodes
            .get_unchecked_mut(index.0.get())
            .take()
            .unwrap_unchecked()
    }

    pub fn push_new_node(&mut self, node: Node<K, T>) -> NodeRef {
        if self.nodes.is_empty() {
            self.nodes.reserve(2);
            self.nodes.push(None);
            self.nodes.push(Some(node));
            NodeRef(unsafe { NonZeroUsize::new_unchecked(1) })
        } else {
            let index = self.nodes.len();
            self.nodes.push(Some(node));
            NodeRef(unsafe { NonZeroUsize::new_unchecked(index) })
        }
    }

    pub fn insert_new_node_right(&mut self, key: K, value: T, node: NodeRef) -> NodeRef {
        let right = self.get(node).right;
        let new_node = Node::new(key, value, node, right);
        let new_node_ref = self.push_new_node(new_node);
        self.get_mut(node).right = new_node_ref;
        self.get_mut(right).left = new_node_ref;
        new_node_ref
    }

    pub unsafe fn push_cyclic_node(&mut self, key: K, value: T) -> NodeRef {
        let new_node_ref;
        if self.nodes.is_empty() {
            new_node_ref = NodeRef(NonZeroUsize::new_unchecked(1));
            self.nodes.reserve(2);
            self.nodes.push(None);
        } else {
            new_node_ref = NodeRef(NonZeroUsize::new_unchecked(self.nodes.len()));
        }
        let node = Node::new(key, value, new_node_ref, new_node_ref);
        self.nodes.push(Some(node));
        new_node_ref
    }

    /// parentのdegreeを更新しない
    pub fn join_child(&mut self, parent: NodeRef, node: NodeRef) {
        let parent_mut = self.get_mut(parent);
        if let Some(child) = parent_mut.child {
            self.insert_right(child, node);
            self.get_mut(child).parent = Some(parent);
            // self.get_mut(parent).degree += 1;
        } else {
            parent_mut.child = Some(node);
            self.link_cyclic(node);
        }
    }

    pub fn link_cyclic(&mut self, node: NodeRef) {
        let node_mut = self.get_mut(node);
        node_mut.right = node;
        node_mut.left = node;
    }

    pub fn insert_right(&mut self, position: NodeRef, node: NodeRef) {
        let right = self.get(position).right;
        let node_mut = self.get_mut(node);
        node_mut.left = position;
        node_mut.right = right;
        self.get_mut(position).right = node;
        self.get_mut(right).left = node;
    }
}

impl<K: Ord, T> NodeArray<K, T> {
    pub unsafe fn remove_max(&mut self, max: NodeRef) -> (Option<NodeRef>, K, T) {
        let node = self.destroy(max);
        debug_assert!(node.parent.is_none());
        let mut buf = [Option::<NodeRef>::None; 64];
        let mut tree = node.right;
        let mut max_degree = 0;
        while tree != max {
            let next = self.get(tree).right;
            let mut degree = self.get(tree).degree;
            while let Some(other) = buf[degree as usize].take() {
                tree = self.merge_tree(tree, other);
                degree += 1;
            }
            max_degree = max_degree.max(degree);
            buf[degree as usize] = Some(tree);
            self.get_mut(tree).degree = degree;
            tree = next;
        }

        if let Some(child) = node.child {
            let mut tree = child;
            while {
                let next = self.get(tree).right;
                let mut degree = self.get(tree).degree;
                while let Some(other) = buf[degree as usize].take() {
                    tree = self.merge_tree(tree, other);
                    degree += 1;
                }
                max_degree = max_degree.max(degree);
                buf[degree as usize] = Some(tree);
                self.get_mut(tree).degree = degree;
                tree = next;
                tree != child
            } {}
        }

        let mut trees = buf[..=max_degree as usize].iter().copied().flatten();
        (
            trees.next().map(|first| {
                let mut max_tree = first;
                let mut prev = first;
                for tree in trees {
                    if self.get(tree).key > self.get(max_tree).key {
                        max_tree = tree;
                    }
                    self.get_mut(prev).right = tree;
                    self.get_mut(tree).left = prev;
                    self.get_mut(tree).parent = None;
                    prev = tree;
                }
                self.get_mut(prev).right = first;
                self.get_mut(first).left = prev;
                self.get_mut(first).parent = None;
                max_tree
            }),
            node.key,
            node.value,
        )
    }

    /// degreeを更新しない
    pub fn merge_tree(&mut self, tree: NodeRef, other: NodeRef) -> NodeRef {
        let (min, max) = if self.get(tree).key < self.get(other).key {
            (tree, other)
        } else {
            (other, tree)
        };
        self.join_child(max, min);
        max
    }
}

#[derive(Debug)]
pub struct Node<K, T> {
    key: K,
    value: T,
    degree: u16,
    damaged: bool,
    left: NodeRef,
    right: NodeRef,
    child: Option<NodeRef>,
    parent: Option<NodeRef>,
}

impl<K, T> Node<K, T> {
    pub fn new(key: K, value: T, left: NodeRef, right: NodeRef) -> Self {
        Self {
            key,
            value,
            degree: 0,
            damaged: false,
            left,
            right,
            child: None,
            parent: None,
        }
    }

    pub fn key_value(&self) -> (&K, &T) {
        (&self.key, &self.value)
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}
/// 有効なノードを参照するインデックス
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeRef(NonZeroUsize);

impl NodeRef {
    pub const fn inner(self) -> NonZeroUsize {
        self.0
    }
}

}

#[derive(Debug)]
pub struct FibonacciHeap<K, T> {
    nodes: NodeArray<K, T>,
    max: Option<NodeRef>,
    len: usize,
}

pub struct Cursor(NonZeroUsize);

impl From<NodeRef> for Cursor {
    fn from(value: NodeRef) -> Self {
        Self(value.inner())
    }
}

impl<K, T> FibonacciHeap<K, T> {
    pub const fn new() -> Self {
        Self {
            nodes: NodeArray::new(),
            max: None,
            len: 0,
        }
    }

    pub fn peek(&self) -> Option<(&K, &T)> {
        self.max
            .map(|node_ref| self.nodes.get(node_ref).key_value())
    }

    pub fn peek_key(&self) -> Option<&K> {
        self.max.map(|node_ref| self.nodes.get(node_ref).key())
    }

    pub fn peek_value(&self) -> Option<&T> {
        self.max.map(|node_ref| self.nodes.get(node_ref).value())
    }

    pub fn cursor_max(&self) -> Option<Cursor> {
        self.max.map(|node_ref| node_ref.into())
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<K: Ord, T> FibonacciHeap<K, T> {
    pub fn push(&mut self, key: K, value: T) -> Cursor {
        let new_node_ref;
        if let Some(max) = self.max {
            let max_key = self.nodes.get(max).key();
            if max_key < &key {
                new_node_ref = self.nodes.insert_new_node_right(key, value, max);
                self.max = Some(new_node_ref);
            } else {
                new_node_ref = self.nodes.insert_new_node_right(key, value, max);
            }
        } else {
            unsafe {
                new_node_ref = self.nodes.push_cyclic_node(key, value);
                self.max = Some(new_node_ref);
            }
        }
        self.len += 1;
        new_node_ref.into()
    }

    pub fn pop(&mut self) -> Option<(K, T)> {
        self.max.map(|max| {
            let (new_max, k, v) = unsafe { self.nodes.remove_max(max) };
            self.max = new_max;
            (k, v)
        })
    }
}

impl<K, T> Default for FibonacciHeap<K, T> {
    fn default() -> Self {
        Self::new()
    }
}


fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        uvb: [(usize, usize, u64); m],
    }

    let mut adj_list = vec![vec![]; n];
    for &(u, v, b) in &uvb {
        adj_list[u - 1].push((v - 1, b));
        adj_list[v - 1].push((u - 1, b));
    }

    let mut heap = FibonacciHeap::new();
    heap.push(Reverse(a[0]), 0);
    let mut dist = vec![u64::MAX; n];
    dist[0] = a[0];
    while let Some((Reverse(w), node)) = heap.pop() {
        if w > dist[node] {
            continue;
        }
        for &(next, b) in &adj_list[node] {
            let next_w = w + b + a[next];
            if next_w < dist[next] {
                dist[next] = next_w;
                heap.push(Reverse(next_w), next);
            }
        }
    }
    for &d in &dist[1..] {
        print!("{} ", d);
    }
    println!();
}
