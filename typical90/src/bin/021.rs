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
    let mut scc = vec![usize::MAX; n];
    let mut j = 0;
    let mut stack = vec![];
    let mut rev_edge = vec![usize::MAX; n];
    let mut visited = vec![0; n];
    let mut iter = 0;
    for i in 0..n {
        if scc[i] != usize::MAX {
            continue;
        }
        iter += 1;
        stack.clear();
        stack.push((i, i));
        while let Some((u, p)) = stack.pop() {
            if scc[u] == usize::MAX {
                scc[u] = j;
                visited[u] = iter;
                j += 1;
                rev_edge[u] = p;
                for &v in &adj_list[u] {
                    stack.push((v, u));
                }
            } else if visited[u] == iter {
                let mut cur = p;
                let scc_id = scc[u];
                while scc[cur] != scc_id {
                    let next = rev_edge[cur];
                    scc[cur] = scc_id;
                    cur = next;
                }
            }
        }
    }
    eprintln!("{:?}", scc);
}
