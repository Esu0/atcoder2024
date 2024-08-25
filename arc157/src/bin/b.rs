use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut s: marker::Bytes,
    }
    let x_count = s.iter().filter(|&&c| c == b'X').count();
    if k > x_count {
        for c in s.iter_mut() {
            match *c {
                b'X' => *c = b'Y',
                b'Y' => *c = b'X',
                _ => unreachable!(),
            }
        }
        k = n - k;
    }
    if s.iter().all(|&c| c == b'X') {
        println!("{}", k.saturating_sub(1));
        return;
    }
    let mut ans = 0u32;
    let mut x_set = BinaryHeap::new();
    for w in s.windows(2) {
        if w[0] == b'Y' && w[1] == b'Y' {
            ans += 1;
        }
    }
    let l = s.iter().position(|&c| c == b'Y').unwrap();
    let r = s.iter().rposition(|&c| c == b'Y').unwrap();
    let mut count = 0u32;
    for &c in &s[l..=r] {
        if c == b'X' {
            count += 1;
        } else {
            if count > 0 {
                x_set.push(Reverse(count));
            }
            count = 0;
        }
    }
    while let Some(Reverse(x)) = x_set.pop() {
        let x = x as usize;
        if k >= x {
            k -= x;
            ans += x as u32 + 1;
        } else {
            break;
        }
    }
    println!("{}", ans + k as u32);
}
