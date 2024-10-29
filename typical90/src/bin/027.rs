use std::collections::HashSet;

use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: [marker::Bytes; n],
    }

    let mut set = HashSet::new();
    let mut count = 1;
    for si in &s {
        if set.insert(&si[..]) {
            println!("{}", count);
        }
        count += 1;
    }
}
