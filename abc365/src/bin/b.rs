use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut b = a.iter().copied().enumerate().collect::<Vec<_>>();
    b.sort_unstable_by_key(|&(_, x)| Reverse(x));
    println!("{}", b[1].0 + 1);
}
