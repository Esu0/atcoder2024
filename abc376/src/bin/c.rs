use std::iter;

use util::upper_bound;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        mut b: [u32; n - 1],
    }
    a.sort_unstable();
    b.sort_unstable();
    let ans = upper_bound(0..=1000000000, |x| {
        let i = b.partition_point(|&bi| bi <= x);
        for (&ai, &bi) in a.iter().zip(b[..i].iter().chain(iter::once(&x)).chain(&b[i..])) {
            if ai > bi {
                return true;
            }
        }
        false
    });
    if ans >= 1000000001 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
