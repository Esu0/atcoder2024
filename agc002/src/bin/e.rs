use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }
    a.sort_unstable_by_key(|&x| Reverse(x));
    let (mut i, mut j) = (0, 0);
    
}
