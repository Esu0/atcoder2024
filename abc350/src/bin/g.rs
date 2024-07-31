use link_cut_tree::{query::Add, LinkCutTree};
use proconio::input;

#[allow(dead_code)]
mod link_cut_tree {
    mod node {
        #![allow(dead_code)]
        use std::{
            collections::HashSet,
            fmt,
            hash::Hash,
            marker::PhantomData,
            ptr::{addr_of, addr_of_mut, NonNull},
        };

        #[derive(Clone)]
        pub struct Node<T, Q, OP> {
            value: T,
            query: Q,
            reverse: bool,
            parent: Option<NodeRef<T, Q, OP>>,
            left: Option<NodeRef<T, Q, OP>>,
            right: Option<NodeRef<T, Q, OP>>,
            _marker: PhantomData<fn() -> OP>,
        }

        #[repr(transparent)]
        pub struct NodeRef<T, Q, OP>(NonNull<Node<T, Q, OP>>);

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Direction {
            Left,
            Right,
        }

        impl Direction {
            pub fn opposite(self) -> Self {
                match self {
                    Left => Right,
                    Right => Left,
                }
            }
        }

        use Direction::*;

        use super::query::{Commutative, Query};

        impl<T, Q, OP> Clone for NodeRef<T, Q, OP> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<T, Q, OP> Copy for NodeRef<T, Q, OP> {}

        impl<T, Q, OP> PartialEq for NodeRef<T, Q, OP> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<T, Q, OP> Eq for NodeRef<T, Q, OP> {}

        impl<T, Q, OP> Hash for NodeRef<T, Q, OP> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state)
            }

            fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
            where
                Self: Sized,
            {
                NonNull::hash_slice(
                    unsafe {
                        std::slice::from_raw_parts(
                            data.as_ptr() as *const NonNull<Node<T, Q, OP>>,
                            data.len(),
                        )
                    },
                    state,
                )
            }
        }

        impl<T, Q, OP> Node<T, Q, OP> {
            pub const fn new(value: T, query: Q) -> Self {
                Self {
                    query,
                    value,
                    reverse: false,
                    parent: None,
                    left: None,
                    right: None,
                    _marker: PhantomData,
                }
            }
        }

        impl<T, Q, OP> Node<T, Q, OP> {
            pub fn child(&self, dir: Direction) -> Option<NodeRef<T, Q, OP>> {
                match dir {
                    Left => self.left,
                    Right => self.right,
                }
            }
        }

        impl<T, Q, OP> NodeRef<T, Q, OP> {
            pub fn new(node: Node<T, Q, OP>) -> Self {
                Self(NonNull::from(Box::leak(Box::new(node))))
            }

            pub fn child(self, dir: Direction) -> Option<Self> {
                unsafe {
                    let ptr = match dir {
                        Left => addr_of!((*self.0.as_ptr()).left),
                        Right => addr_of!((*self.0.as_ptr()).right),
                    };
                    ptr.read()
                }
            }

            pub fn parent(self) -> Option<Self> {
                unsafe { addr_of!((*self.0.as_ptr()).parent).read() }
            }

            pub fn set_child(self, dir: Direction, child: Option<Self>) -> Option<Self> {
                unsafe {
                    let ptr = match dir {
                        Left => addr_of_mut!((*self.0.as_ptr()).left),
                        Right => addr_of_mut!((*self.0.as_ptr()).right),
                    };
                    let old = ptr.read();
                    ptr.write(child);
                    old
                }
            }

            pub fn set_parent(self, parent: Option<Self>) -> Option<Self> {
                unsafe {
                    let ptr = addr_of_mut!((*self.0.as_ptr()).parent);
                    let old = ptr.read();
                    ptr.write(parent);
                    old
                }
            }

            /// selfを親、childを子とし、selfとchildを双方向にリンクする
            ///
            /// # Returns
            /// もともとselfの子だったノードと、もともとchildの親だったノード
            pub fn link_child(
                self,
                dir: Direction,
                child: Option<Self>,
            ) -> (Option<Self>, Option<Self>) {
                let old_parent = child.and_then(|child| child.set_parent(Some(self)));
                let old_child = self.set_child(dir, child);
                (old_child, old_parent)
            }

            /// dirの方向に木の回転を行う。childはdirの反対方向の子であると仮定して回転を行うことに注意
            /// selfとchildが双方向にリンクされている必要はない
            pub fn rot_child(self, child: Self, dir: Direction) -> Option<Self> {
                let (old_child, old_parent) = child.link_child(dir, Some(self));
                self.link_child(dir.opposite(), old_child);
                child.set_parent(old_parent);
                old_parent
            }

            pub fn rot(self, dir: Direction) -> Option<(Self, Option<Self>)> {
                self.child(dir.opposite())
                    .map(|child| (child, self.rot_child(child, dir)))
            }

            /// 回転を行わなかった場合None、回転を行った場合Some((新しい親, 回転した方向))
            pub fn rot_parent(self) -> Option<(Option<Self>, Direction)> {
                self.parent().and_then(|parent| {
                    parent
                        .direction(self)
                        .map(|dir| (parent.rot_child(self, dir.opposite()), dir.opposite()))
                })
            }

            pub fn direction(self, child: Self) -> Option<Direction> {
                if self.child(Left) == Some(child) {
                    Some(Left)
                } else if self.child(Right) == Some(child) {
                    Some(Right)
                } else {
                    None
                }
            }

            pub fn parent_and_direction(self) -> Option<(Self, Option<Direction>)> {
                self.parent().map(|parent| (parent, parent.direction(self)))
            }

            // pub fn insert_val(self, dir: Direction, value: T) -> Self
            // where
            //     T: Clone,
            // {
            //     let child = self.child(dir);
            //     let mut new_node = Node {
            //         query: value.clone(),
            //         value,
            //         reverse: false,
            //         parent: Some(self),
            //         left: None,
            //         right: None,
            //         _marker: PhantomData,
            //     };
            //     match dir {
            //         Left => new_node.left = child,
            //         Right => new_node.right = child,
            //     };
            //     let new_node_ref = NodeRef::new(new_node);
            //     if let Some(child) = child {
            //         child.set_parent(Some(new_node_ref));
            //     }
            //     self.set_child(dir, Some(new_node_ref));
            //     new_node_ref
            // }

            pub fn dfs(self, mut f: impl FnMut(Self, Self)) {
                let mut stack = vec![self];
                let mut visited = HashSet::from([self]);
                while let Some(node) = stack.pop() {
                    let left = node.child(Left);
                    let right = node.child(Right);
                    let parent = node.parent();
                    for next in [left, right, parent].into_iter().flatten() {
                        f(node, next);
                        if visited.insert(next) {
                            stack.push(next);
                        }
                    }
                }
            }

            pub fn node(&self) -> &Node<T, Q, OP> {
                unsafe { self.0.as_ref() }
            }

            pub fn node_mut(&mut self) -> &mut Node<T, Q, OP> {
                unsafe { self.0.as_mut() }
            }

            pub fn val(&self) -> &T {
                unsafe { self.borrow_val() }
            }

            pub unsafe fn borrow_val<'a>(self) -> &'a T {
                unsafe {
                    let ptr = addr_of!((*self.0.as_ptr()).value);
                    &*ptr
                }
            }

            pub fn val_mut(&mut self) -> &mut T {
                unsafe {
                    let ptr = addr_of_mut!((*self.0.as_ptr()).value);
                    &mut *ptr
                }
            }

            pub fn query(&self) -> &Q {
                unsafe { self.borrow_query() }
            }

            pub unsafe fn borrow_query<'a>(self) -> &'a Q {
                unsafe {
                    let ptr = addr_of!((*self.0.as_ptr()).query);
                    &*ptr
                }
            }

            pub fn query_mut(&mut self) -> &mut Q {
                unsafe {
                    let ptr = addr_of_mut!((*self.0.as_ptr()).query);
                    &mut *ptr
                }
            }

            pub fn reverse(self) {
                unsafe {
                    let reverse_ptr = addr_of_mut!((*self.0.as_ptr()).reverse);
                    let left_ptr = addr_of_mut!((*self.0.as_ptr()).left);
                    let right_ptr = addr_of_mut!((*self.0.as_ptr()).right);
                    *reverse_ptr = !*reverse_ptr;
                    let left = left_ptr.read();
                    left_ptr.write(right_ptr.read());
                    right_ptr.write(left);
                }
            }

            pub fn push(mut self) -> bool {
                let node = self.node_mut();
                if node.reverse {
                    if let Some(left) = node.left {
                        left.reverse();
                    }
                    if let Some(right) = node.right {
                        right.reverse();
                    }
                    node.reverse = false;
                    true
                } else {
                    false
                }
            }

            pub fn push_either(mut self, dir: Direction) -> bool {
                let node = self.node_mut();
                if node.reverse {
                    if let Some(child) = node.child(dir) {
                        child.reverse();
                    }
                    node.reverse = false;
                    true
                } else {
                    false
                }
            }

            pub unsafe fn drop(self) {
                drop(Box::from_raw(self.0.as_ptr()));
            }
        }

        impl<T, Q, OP: Query<ValT = T, QValT = Q> + Commutative> NodeRef<T, Q, OP> {
            pub fn update_from_child(mut self, op: &OP) {
                let Node {
                    value: ref val,
                    query: query_mut,
                    left,
                    right,
                    ..
                } = self.node_mut();
                match (*left, *right) {
                    (Some(left), Some(right)) => {
                        *query_mut = op.op(&op.op_left(left.query(), val), right.query())
                    }
                    (Some(left), None) => *self.query_mut() = op.op_left(left.query(), val),
                    (None, Some(right)) => *self.query_mut() = op.op_right(val, right.query()),
                    (None, None) => *self.query_mut() = op.val_to_query(val),
                };
            }

            /// selfのクエリの値は更新しない
            ///
            /// # Returns
            /// * 元のルートノードの親ノードと元のルートノード
            pub fn splay(self, op: &OP) -> (Option<Self>, Self) {
                let mut pd = self.parent_and_direction();
                let mut prev_p = self;
                while let Some((p, Some(dir1))) = pd {
                    if let Some((gp, Some(dir2))) = p.parent_and_direction() {
                        let dir1 = if gp.push() { dir1.opposite() } else { dir1 };
                        if p.push_either(dir1.opposite()) {
                            self.reverse();
                        }
                        self.push();
                        if dir1 == dir2 {
                            let next_p = gp.rot_child(p, dir1.opposite());
                            p.rot_child(self, dir1.opposite());
                            pd = next_p.map(|p| (p, p.direction(gp)));
                        } else {
                            p.rot_child(self, dir2);
                            let next_p = gp.rot_child(self, dir1);
                            pd = next_p.map(|p| (p, p.direction(gp)));
                        }
                        gp.update_from_child(op);
                        p.update_from_child(op);
                        prev_p = gp;
                        // self.update_from_child(query);
                    } else {
                        if p.push_either(dir1.opposite()) {
                            self.reverse();
                        }
                        self.push();
                        let ret = p.rot_child(self, dir1.opposite());
                        p.update_from_child(op);
                        // self.update_from_child(op);
                        return (ret, p);
                    }
                }
                // self.update_from_child(op);
                (pd.map(|(p, _)| p), prev_p)
            }

            /// # Returns
            ///
            /// * タプルの二つ目は元のルートノード
            /// * タプルの一つ目は元のルートノードと`self`のLCA
            pub fn expose(self, op: &OP) -> (Self, Self) {
                let mut prev = None;
                let mut current = self;
                let mut prev_root;
                while let Some(p) = {
                    let (p, prev_parent) = current.splay(op);
                    prev_root = prev_parent;
                    p
                } {
                    current.push();
                    current.set_child(Right, prev);
                    prev = Some(current);
                    current = p;
                }
                current.push();
                current.set_child(Right, prev);
                self.splay(op);
                // self.update_from_child(op);
                (current, prev_root)
            }

            pub fn cut(self, op: &OP) {
                self.expose(op);
                if let Some(node) = self.child(Left) {
                    self.set_child(Left, None);
                    node.set_parent(None);
                }
            }

            pub fn cut_checked(self, parent: Self, op: &OP) -> bool {
                parent.evert(op);
                self.expose(op);
                if Some(parent) == self.child(Left) && parent.child(Right).is_none() {
                    self.set_child(Left, None);
                    parent.set_parent(None);
                    true
                } else {
                    false
                }
            }

            /// nodeを親としてパスselfをくっつける
            pub fn link(self, node: Self, op: &OP) {
                self.expose(op);
                self.update_from_child(op);
                node.expose(op);
                node.link_child(Right, Some(self));
                node.update_from_child(op);
            }

            pub fn link_checked(self, parent: Self, op: &OP) -> bool {
                self.evert(op);
                if self != parent.expose(op).1 {
                    self.set_parent(Some(parent));
                    true
                } else {
                    false
                }
            }

            pub fn update_and_get_query(&self, op: &OP) -> &Q {
                self.update_from_child(op);
                self.query()
            }

            pub fn evert(self, op: &OP) {
                self.expose(op);
                self.reverse();
            }
        }

        pub struct Tree<T, Q, OP>(Option<NodeRef<T, Q, OP>>);

        impl<T, Q, OP> Clone for Tree<T, Q, OP> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<T, Q, OP> Copy for Tree<T, Q, OP> {}

        impl<T: fmt::Display, Q, OP> Tree<T, Q, OP> {
            fn fmt_rec(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
                if let Some(node) = self.0 {
                    let node = node.node();
                    Tree(node.right).fmt_rec(f, depth + 1)?;
                    writeln!(f, "{:indent$}{}", "", node.value, indent = depth * 2)?;
                    Tree(node.left).fmt_rec(f, depth + 1)?;
                }
                Ok(())
            }
        }

        impl<T: fmt::Display, Q, OP> fmt::Display for Tree<T, Q, OP> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.fmt_rec(f, 0)
            }
        }

        impl<T, Q, OP> From<NodeRef<T, Q, OP>> for Tree<T, Q, OP> {
            fn from(value: NodeRef<T, Q, OP>) -> Self {
                Tree(Some(value))
            }
        }

        impl<T: fmt::Debug, Q, OP> NodeRef<T, Q, OP> {
            fn debug_ancestor(self) {
                let mut current = self;
                print!("{:?}", current.node().value);
                while let Some(parent) = current.parent() {
                    current = parent;
                    print!(" -> {:?}", current.node().value);
                }
                println!();
            }
        }
    }
    pub mod query {
        use std::{marker::PhantomData, ops};

        pub trait Query {
            type ValT;
            type QValT;
            const IDENT: Self::QValT;
            fn op_left(&self, a: &Self::QValT, b: &Self::ValT) -> Self::QValT;
            fn op_right(&self, a: &Self::ValT, b: &Self::QValT) -> Self::QValT;
            fn op(&self, a: &Self::QValT, b: &Self::QValT) -> Self::QValT;
            fn val_to_query(&self, val: &Self::ValT) -> Self::QValT {
                self.op_left(&Self::IDENT, val)
            }
        }

        pub trait Commutative: Query {}

        pub struct Noop<T>(PhantomData<fn() -> T>);

        impl<T> Query for Noop<T> {
            type ValT = T;
            type QValT = ();
            const IDENT: Self::QValT = ();
            fn op_left(&self, _: &Self::QValT, _: &Self::ValT) -> Self::QValT {}
            fn op_right(&self, _: &Self::ValT, _: &Self::QValT) -> Self::QValT {}
            fn op(&self, _: &Self::QValT, _: &Self::QValT) -> Self::QValT {}
        }

        pub struct Add<T>(PhantomData<fn() -> T>);

        impl<T> Add<T> {
            pub const fn new() -> Self {
                Self(PhantomData)
            }
        }

        impl<T> Default for Add<T> {
            fn default() -> Self {
                Self::new()
            }
        }

        pub trait HasZero {
            const ZERO: Self;
        }

        macro_rules! impl_has_zero {
        ($zero:expr, $($t:ty),*) => {
            $(
                impl HasZero for $t {
                    const ZERO: Self = $zero;
                }
            )*
        };
    }

        impl_has_zero!(0, i8, i16, i32, i64, i128);
        impl_has_zero!(0, u8, u16, u32, u64, u128);
        impl_has_zero!(0., f32, f64);

        impl<T: ops::Add<Output = T> + HasZero + Clone> Query for Add<T> {
            type ValT = T;
            type QValT = T;
            const IDENT: Self::QValT = T::ZERO;
            fn op_left(&self, a: &Self::QValT, b: &Self::ValT) -> Self::QValT {
                a.clone() + b.clone()
            }
            fn op_right(&self, a: &Self::ValT, b: &Self::QValT) -> Self::QValT {
                a.clone() + b.clone()
            }
            fn op(&self, a: &Self::QValT, b: &Self::QValT) -> Self::QValT {
                a.clone() + b.clone()
            }
        }

        impl<T> Commutative for Add<T> where Add<T>: Query {}
    }

    use node::{Node, NodeRef};
    use query::{Commutative, Query};

    struct NodeValue<T> {
        value: T,
        id: usize,
    }

    struct NodeOP<OP> {
        inner: OP,
    }

    pub struct LinkCutTree<T, OP> {
        nodes: Vec<NodeRef<NodeValue<T>, T, NodeOP<OP>>>,
        op: NodeOP<OP>,
    }

    impl<T, OP> Drop for LinkCutTree<T, OP> {
        fn drop(&mut self) {
            for &node in self.nodes.iter() {
                unsafe {
                    node.drop();
                }
            }
        }
    }

    impl<T, OP> LinkCutTree<T, OP> {
        pub const fn new(op: OP) -> Self {
            Self {
                nodes: Vec::new(),
                op: NodeOP { inner: op },
            }
        }
    }

    impl<T, OP: Query<ValT = T, QValT = T>> Query for NodeOP<OP> {
        type ValT = NodeValue<T>;
        type QValT = T;
        const IDENT: Self::QValT = OP::IDENT;
        fn op_left(&self, a: &Self::QValT, b: &Self::ValT) -> Self::QValT {
            self.inner.op_left(a, &b.value)
        }
        fn op_right(&self, a: &Self::ValT, b: &Self::QValT) -> Self::QValT {
            self.inner.op_right(&a.value, b)
        }
        fn op(&self, a: &Self::QValT, b: &Self::QValT) -> Self::QValT {
            self.inner.op(a, b)
        }
    }

    impl<OP> Commutative for NodeOP<OP>
    where
        OP: Commutative,
        NodeOP<OP>: Query,
    {
    }

    impl<T: Clone, OP: Query<ValT = T, QValT = T> + Commutative> LinkCutTree<T, OP> {
        pub fn from_iter<I>(op: OP, iter: I) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            Self {
                nodes: iter
                    .into_iter()
                    .enumerate()
                    .map(|(id, value)| {
                        NodeRef::new(Node::new(
                            NodeValue {
                                value: value.clone(),
                                id,
                            },
                            value,
                        ))
                    })
                    .collect(),
                op: NodeOP { inner: op },
            }
        }

        pub fn get(&self, i: usize) -> Option<&T> {
            self.nodes
                .get(i)
                .map(|&node| unsafe { &node.borrow_val().value })
        }

        pub fn cut(&mut self, i: usize, j: usize) -> bool {
            self.nodes[i].cut_checked(self.nodes[j], &self.op)
        }

        pub fn link(&mut self, i: usize, j: usize) -> bool {
            self.nodes[i].link_checked(self.nodes[j], &self.op)
        }

        pub fn lca(&mut self, p: usize, i: usize, j: usize) -> Option<usize> {
            let p = self.nodes[p];
            let i = self.nodes[i];
            let j = self.nodes[j];
            p.evert(&self.op);
            if p != i.expose(&self.op).1 {
                return None;
            }
            let (lca, prev_p) = j.expose(&self.op);
            if prev_p == i {
                Some(lca.val().id)
            } else {
                None
            }
        }

        pub fn path_query(&mut self, i: usize, j: usize) -> Option<&T> {
            let i = self.nodes[i];
            let j = self.nodes[j];
            i.evert(&self.op);
            if i != j.expose(&self.op).1 {
                return None;
            }
            j.update_from_child(&self.op);
            Some(unsafe { j.borrow_query() })
        }

        pub fn parent(&mut self, root: usize, i: usize) -> Option<usize> {
            if root == i {
                return None;
            }
            let root = self.nodes[root];
            let i = self.nodes[i];
            root.evert(&self.op);
            if root != i.expose(&self.op).1 {
                return None;
            }
            use node::Direction::*;
            let mut parent = i.child(Left).unwrap();
            while let Some(next) = parent.child(Right) {
                parent = next;
            }
            Some(parent.val().id)
        }
    }

    impl<T, Q: Default> Default for LinkCutTree<T, Q> {
        fn default() -> Self {
            Self::new(Q::default())
        }
    }
}
const MODULO: usize = 998244353;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut x = 0;
    let mut lct = LinkCutTree::from_iter(Add::new(), vec![0u32; n]);
    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
        }
        let a = 1 + (a * (1 + x) % MODULO) % 2;
        let b = (b * (1 + x) % MODULO) % n;
        let c = (c * (1 + x) % MODULO) % n;
        match a {
            1 => {
                lct.link(b, c);
            }
            2 => {
                if let Some(p) = lct.parent(b, c) {
                    if lct.parent(b, p) == Some(b) {
                        x = p + 1;
                    } else {
                        x = 0;
                    }
                } else {
                    x = 0;
                }
                println!("{}", x);
            }
            _ => unreachable!(),
        }
    }
}
