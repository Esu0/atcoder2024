use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(usize, usize, i64); m],
    }
    let mut adj_list = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        adj_list[u - 1].push((v - 1, w));
        adj_list[v - 1].push((u - 1, -w));
    }
    let mut visited = vec![false; n];
    let mut l = 0;
    let mut stack = vec![];
    let mut ans = vec![0; n];
    while l < n {
        stack.clear();
        stack.push(l);
        while let Some(u) = stack.pop() {
            if visited[u] {
                continue;
            }
            let xu = ans[u];
            visited[u] = true;
            while l < n && visited[l] {
                l += 1;
            }
            for &(v, w) in &adj_list[u] {
                if !visited[v] {
                    ans[v] = xu + w;
                    stack.push(v);
                }
            }
        }
    }
    ans.iter().for_each(|&x| print!("{} ", x));
    println!();
}
