use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        m: usize,
        b: [u64; m],
        l: usize,
        c: [u64; l],
        q: usize,
        x: [u64; q],
    }
    let mut set = HashSet::new();
    for &ai in &a {
        for &bi in &b {
            for &ci in &c {
                set.insert(ai + bi + ci);
            }
        }
    }
    for &xi in &x {
        if set.contains(&xi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
