use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    let mut dp = vec![vec![0; 2000]; 2000];
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            c: [u8; n],
            uv: [(usize, usize); m],
        }
        let mut adj_list = vec![vec![]; n];
        for &(u, v) in &uv {
            adj_list[u - 1].push(v - 1);
            adj_list[v - 1].push(u - 1);
        }
        dp[..n].iter_mut().for_each(|row| row.fill(u32::MAX));
        dp[0][n - 1] = 0;
        let mut q = VecDeque::from([0]);
    }
}
