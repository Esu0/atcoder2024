use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let mut rows = vec![(0..w).collect::<BTreeSet<_>>(); h];
    let mut cols = vec![(0..h).collect::<BTreeSet<_>>(); w];
    for _ in 0..q {
        input! {
            r: usize,
            c: usize,
        }
        let r = r - 1;
        let c = c - 1;
        if rows[r].remove(&c) {
            cols[c].remove(&r);
        } else {
            if let Some(&w) = rows[r].range(..c).next_back() {
                rows[r].remove(&w);
                cols[w].remove(&r);
            }
            if let Some(&w) = rows[r].range(c + 1..).next() {
                rows[r].remove(&w);
                cols[w].remove(&r);
            }
            if let Some(&w) = cols[c].range(..r).next_back() {
                cols[c].remove(&w);
                rows[w].remove(&c);
            }
            if let Some(&w) = cols[c].range(r + 1..).next() {
                cols[c].remove(&w);
                rows[w].remove(&c);
            }
        }
    }
    let ans = rows.iter().map(|row| row.len()).sum::<usize>();
    println!("{}", ans);
}
