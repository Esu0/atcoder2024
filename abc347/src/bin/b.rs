use std::collections::HashSet;

use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let mut set = HashSet::new();
    for l in 0..s.len() {
        for r in l + 1..=s.len() {
            set.insert(&s[l..r]);
        }
    }
    println!("{}", set.len());
}
