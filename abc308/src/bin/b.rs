use std::collections::HashMap;

use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [marker::Bytes; n],
        d: [marker::Bytes; m],
        p: [u32; m + 1],
    }
    let map = d.iter().map(|v| &v[..]).zip(p[1..].iter().copied()).collect::<HashMap<_, _>>();
    let ans = c.iter().map(|ci| map.get(&ci[..]).copied().unwrap_or(p[0])).sum::<u32>();
    println!("{}", ans);
}
