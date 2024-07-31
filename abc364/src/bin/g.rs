use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        abc: [(usize, usize, u64); m],
    }

    let mut adj_list = vec![vec![]; n];
    for &(a, b, c) in &abc {
        adj_list[a - 1].push((b - 1, c));
        adj_list[b - 1].push((a - 1, c));
    }

    let mut dp = vec![vec![u64::MAX; n]; 1 << (k - 1)];
    let mut b1 = 1;
    for i in 0..k - 1 {
        dp[b1][i] = 0;
        b1 <<= 1;
    }
    let mut confirmed = vec![false; n];
    let mut queue = BinaryHeap::new();
    for i in 1usize..(1 << (k - 1)) {
        for v in 0..n {
            let mut j = (i - 1) & i;
            let mut min = u64::MAX;
            while j > 0 {
                let new = dp[j][v].saturating_add(dp[i ^ j][v]);
                if new < min {
                    min = new;
                }
                j = (j - 1) & i;
            }
            let prev = dp[i][v];
            if min < prev {
                dp[i][v] = min;
            }
        }

        confirmed.fill(false);
        queue.clear();
        queue.extend(dp[i].iter().copied().enumerate().map(|(i, c)| (Reverse(c), i)));
        while let Some((Reverse(c), v)) = queue.pop() {
            if confirmed[v] {
                continue;
            }
            confirmed[v] = true;
            dp[i][v] = c;
            for &(u, d) in &adj_list[v] {
                if !confirmed[u] {
                    queue.push((Reverse(c.saturating_add(d)), u));
                }
            }
        }
    }
    eprintln!("{:?}", dp);
    for &ans in &dp[(1 << (k - 1)) - 1][k - 1..] {
        println!("{}", ans);
    }
}
