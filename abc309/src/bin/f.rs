use std::{cmp::Reverse, collections::HashMap};

use proconio::input;
use segtree::{operation, Segtree};

fn main() {
    input! {
        n: usize,
        hwd: [(u32, u32, u32); n],
    }
    let mut hwd = hwd
        .iter()
        .map(|&(h, w, d)| {
            let mut a = [h, w, d];
            a.sort_unstable();
            a
        })
        .collect::<Vec<_>>();
    hwd.sort_unstable_by_key(|a| (a[0], Reverse(a[1])));

    let mut w = hwd.iter().map(|&[_, w, _]| w).collect::<Vec<_>>();
    w.sort_unstable();
    w.dedup();
    let map = w.iter().enumerate().map(|(i, &w)| (w, i)).collect::<HashMap<_, _>>();
    hwd.iter_mut().for_each(|[_, w, _]| *w = map[&*w] as u32);

    // let mut d = hwd.iter().map(|&[_, _, d]| d).collect::<Vec<_>>();
    // d.sort_unstable();
    // d.dedup();
    // let map = d.iter().enumerate().map(|(i, &d)| (d, i)).collect::<HashMap<_, _>>();
    // hwd.iter_mut().for_each(|[_, _, d]| *d = map[&*d] as u32);

    let mut segtree = (0..n).map(|_| u32::MAX).collect::<Segtree<_, operation::Min<_>>>();
    for &[_, w, d] in &hwd {
        let w = w as usize;
        if segtree.query(..w) < d {
            println!("Yes");
            return;
        }
        let mut r = segtree.get_mut(w);
        if *r > d {
            *r = d;
        }
    }
    println!("No");
}
