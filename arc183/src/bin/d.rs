use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut adj_list = vec![vec![]; n];
    for &(a, b) in &ab {
        adj_list[a - 1].push(b - 1);
        adj_list[b - 1].push(a - 1);
    }
    let root = 0;
    let mut parent = vec![usize::MAX; n];
    let mut size = vec![1u32; n];
    let mut stack = vec![root];
    parent[root] = root;
    while let Some(u) = stack.pop() {
        for &v in &adj_list[u] {
            if parent[v] != usize::MAX {
                continue;
            }
            parent[v] = u;
            stack.push(v);
        }
    }
    stack.push(root);
    let mut stack = vec![(root, 0)];
    while let Some((u, i)) = stack.pop() {
        let v_set = &adj_list[u];
        if let Some(&v) = v_set.get(i) {
            stack.push((u, i + 1));
            if v == parent[u] {
                continue;
            }
            stack.push((v, 0));
        } else {
            for &v in v_set {
                if v == parent[u] {
                    continue;
                }
                size[u] += size[v];
            }
        }
    }
    // eprintln!("{:?}", size);

    let mut g = 0;
    loop {
        let (next_g, sz) = adj_list[g]
            .iter()
            .filter(|&&v| parent[g] != v)
            .map(|&v| (v, size[v]))
            .max_by_key(|&(_, sz)| sz)
            .unwrap();
        g = next_g;
        if sz * 2 <= n as u32 {
            break;
        }
    }
    eprintln!("{}", g);
}
