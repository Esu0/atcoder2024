use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, u64); m],
    }
    let mut adj_list = vec![vec![]; n];
    for &(ai, bi, ci) in &abc {
        adj_list[ai - 1].push((bi - 1, ci));
        adj_list[bi - 1].push((ai - 1, ci));
    }

    let mut dist0 = vec![u64::MAX; n];
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(d), node)) = heap.pop() {
        if dist0[node] != u64::MAX {
            continue;
        }
        dist0[node] = d;
        for &(next, cost) in &adj_list[node] {
            let new_d = d + cost;
            if dist0[next] == u64::MAX {
                heap.push((Reverse(new_d), next));
            }
        }
    }

    let mut distn = vec![u64::MAX; n];
    heap.clear();
    heap.push((Reverse(0), n - 1));
    while let Some((Reverse(d), node)) = heap.pop() {
        if distn[node] != u64::MAX {
            continue;
        }
        distn[node] = d;
        for &(next, cost) in &adj_list[node] {
            let new_d = d + cost;
            if distn[next] == u64::MAX {
                heap.push((Reverse(new_d), next));
            }
        }
    }
    for (&d0, &dn) in dist0.iter().zip(&distn) {
        println!("{}", d0 + dn);
    }
}
