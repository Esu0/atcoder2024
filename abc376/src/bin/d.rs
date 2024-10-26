use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut adj_list = vec![vec![]; n];

    for &(a, b) in &ab {
        adj_list[a - 1].push(b - 1);
    }

    let mut queue = VecDeque::from([(0, 0u32)]);
    let mut visited = vec![false; n];
    while let Some((u, dep)) = queue.pop_front() {
        if u == 0 && dep != 0 {
            println!("{}", dep);
            return;
        }
        if visited[u] {
            continue;
        }
        visited[u] = true;

        for &v in &adj_list[u] {
            queue.push_back((v, dep + 1));
        }
    }
    println!("-1");
}
