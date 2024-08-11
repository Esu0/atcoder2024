use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut adj_list = vec![vec![]; n];
    for (a, b) in &ab {
        adj_list[a - 1].push(b - 1);
        adj_list[b - 1].push(a - 1);
    }

    let mut dist = vec![u32::MAX; n];
    dist[0] = 0;
    let mut queue = VecDeque::from([0]);
    while let Some(u) = queue.pop_front() {
        let dep = dist[u];
        for &v in &adj_list[u] {
            if dist[v] != u32::MAX {
                continue;
            }
            dist[v] = dep + 1;
            queue.push_back(v);
        }
    }
    // eprintln!("{:?}", dist);

    let max_dist_v = dist.iter().enumerate().max_by_key(|&(_, d)| d).unwrap().0;

    dist.fill(u32::MAX);
    dist[max_dist_v] = 0;
    queue.clear();
    queue.push_back(max_dist_v);
    while let Some(u) = queue.pop_front() {
        let dep = dist[u];
        for &v in &adj_list[u] {
            if dist[v] != u32::MAX {
                continue;
            }
            dist[v] = dep + 1;
            queue.push_back(v);
        }
    }
    println!("{}", dist.iter().max().unwrap() + 1);
}
