use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut p: [u64; n],
        l: [u64; m],
        d: [u64; m],
    }
    p.sort_unstable();
    let mut ld = l.iter().copied().zip(d.iter().copied()).collect::<Vec<_>>();
    ld.sort_unstable_by_key(|&(li, _)| li);
    let mut heap = BinaryHeap::new();
    let mut j = 0;
    let mut ans = 0;
    for &pi in &p {
        while j < m && ld[j].0 <= pi {
            heap.push(ld[j].1);
            j += 1;
        }
        ans += pi - heap.pop().unwrap_or_default();
    }
    println!("{}", ans);
}
