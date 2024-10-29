use std::cmp::Reverse;

use proconio::input;
use segtree::{operation, Segtree};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(usize, usize); m],
    }
    lr.sort_unstable_by_key(|&(l, r)| (r, Reverse(l)));
    let mut segtree = (0..n).map(|_| 0).collect::<Segtree<i64, operation::Add<_>>>();
    let mut ans = 0;
    for &(l, r) in &lr {
        // eprintln!("l = {}, r = {}", l, r);
        // eprintln!("{:?}", &segtree[..]);
        ans += segtree.query(l - 1..r);
        *segtree.get_mut(l - 1) += -1;
        *segtree.get_mut(r - 2) += 1;
    }
    println!("{}", ans);
}
