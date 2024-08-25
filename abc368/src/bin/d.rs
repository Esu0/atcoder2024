use std::collections::{HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n - 1],
        v: [usize; k],
    }
    let mut adj_list = vec![vec![]; n];
    for &(a, b) in &ab {
        adj_list[a - 1].push(b - 1);
        adj_list[b - 1].push(a - 1);
    }

    let v1 = v[0] - 1;
    let mut prev_v = vec![usize::MAX; n];
    prev_v[v1] = v1;
    let mut queue = VecDeque::from([v1]);
    while let Some(u) = queue.pop_front() {
        for &v in &adj_list[u] {
            if prev_v[v] == usize::MAX {
                prev_v[v] = u;
                queue.push_back(v);
            }
        }
    }

    let mut v_set = HashSet::new();
    for &vi in &v {
        v_set.insert(vi - 1);
        let mut vi = vi - 1;
        while !v_set.contains(&prev_v[vi]) && prev_v[vi] != vi {
            vi = prev_v[vi];
            v_set.insert(vi);
        }
    }
    println!("{}", v_set.len());
}
