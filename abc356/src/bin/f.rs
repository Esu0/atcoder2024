use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        q: usize,
        k: u64,
    }

    let mut out_query = vec![];
    let mut query = vec![];
    let mut set = HashMap::new();
    let mut ti = 0;
    for _ in 0..q {
        input! {
            t: u8,
            x: u64,
        }
        match t {
            1 => {
                if let Some(t0) = set.insert(x, ti) {
                    query.push((x, t0, ti));
                }
                ti += 1;
            }
            2 => out_query.push((query.len(), x)),
            _ => unreachable!(),
        }
    }
    let mut segtree = vec![vec![]; ti];
}
