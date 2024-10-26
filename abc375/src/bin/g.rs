use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, u64); m],
    }
    let mut adj_list = vec![vec![]; n];
    for &(a, b, c) in &abc {
        adj_list[a - 1].push((b - 1, c));
        adj_list[b - 1].push((a - 1, c));
    }

    let mut dist = vec![u64::MAX; n];
    let mut h = vec![vec![]; n];
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0, usize::MAX));
    while let Some((Reverse(d), u, prev_u)) = queue.pop() {
        if dist[u] == d {
            h[prev_u].push(u);
            continue;
        } else if dist[u] != u64::MAX {
            continue;
        }
        dist[u] = d;
        if prev_u != usize::MAX {
            h[prev_u].push(u);
        }
        for &(v, c) in &adj_list[u] {
            queue.push((Reverse(d + c), v, u));
        }
    }

    // eprintln!("{:?}", h);
    // TODO: 橋を列挙したい

}
