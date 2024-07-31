use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut lrc: [(usize, usize, u64); q],
    }
    let mut unconnected = (0..n - 1).collect::<BTreeSet<_>>();
    let mut buf = Vec::new();
    lrc.sort_unstable_by_key(|&(_, _, c)| c);
    let mut sum = 0;
    for &(l, r, c) in &lrc {
        buf.clear();
        sum += c;
        for &i in unconnected.range(l - 1..r - 1) {
            buf.push(i);
            sum += c;
        }
        for &i in &buf {
            unconnected.remove(&i);
        }
    }
    if unconnected.is_empty() {
        println!("{}", sum);
    } else {
        println!("-1");
    }
}
