use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }
    let set = a.iter().copied().filter(|&x| x <= k).collect::<BTreeSet<_>>();
    // eprintln!("{:?}", set);
    let ans = k * (k + 1) / 2 - set.iter().sum::<u64>();
    println!("{}", ans);
}
