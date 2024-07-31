use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
        mut a: [u64; n],
        mut b: [u64; n],
    }
    a.sort_unstable_by_key(|&x| Reverse(x));
    b.sort_unstable_by_key(|&x| Reverse(x));
    let mut sum = 0;
    let mut count1 = 0u32;
    for &ai in &a {
        sum += ai;
        count1 += 1;
        if sum > x {
            break;
        }
    }
    let mut sum = 0;
    let mut count2 = 0u32;
    for &bi in &b {
        sum += bi;
        count2 += 1;
        if sum > y {
            break;
        }
    }
    println!("{}", count1.min(count2));
}
