use proconio::{input, marker};
use segtree::{operation, Segtree};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: marker::Bytes,
    }
    let seg_min = Segtree::from_iter_op(s.iter().copied(), operation::min());
    // let seg_max = Segtree::from_iter_op(s.iter().copied(), operation::max());
    let mut ans = Vec::with_capacity(k);
    let mut l = 0;
    for i in 0..k {
        let r = n - k + 1 + i;
        let c = seg_min.query(l..r);
        ans.push(c);
        l = seg_min.upper_bound(l, |&x| x > c) + 1;
    }
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
