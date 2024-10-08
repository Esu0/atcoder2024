use std::collections::BinaryHeap;

use proconio::input;

macro_rules! chmax {
    ($d:expr, $s:expr) => {
        $d = $d.max($s);
    };
}

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let mut queues = vec![BinaryHeap::new(); w];
    for &(wi, vi) in &wv {
        queues[wi - 1].push((vi as i64 - 1, vi, 1usize));
    }
    // dp[i][j]: 重さがi以下の品物から選んで重さがjになるときの価値の最大値
    let mut dp = vec![vec![i64::MIN; w + 1]; w + 1];
    dp[0][0] = 0;
    let mut tmp = vec![0; w + 1];
    for i in 1..=w {
        tmp.clone_from_slice(&dp[i - 1]);
        let mut k = i;
        if !queues[i - 1].is_empty() {
            let mut v = 0;
            while k <= w {
                let (vs, vu, m) = queues[i - 1].pop().unwrap();
                v += vs;
                queues[i - 1].push((vu as i64 - (2 * m + 1) as i64, vu, m + 1));
                // eprintln!("{:?}", queues);
                for (l, tmpj) in tmp[k..].iter_mut().enumerate() {
                    chmax!(*tmpj, dp[i - 1][l].saturating_add(v));
                }
                k += i;
            }
        }
        dp[i].clone_from(&tmp);
    }
    // eprintln!("{:?}", dp);
    let ans = *dp[w].iter().max().unwrap();
    println!("{}", ans);
}
