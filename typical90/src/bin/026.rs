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

    let mut label = vec![usize::MAX; n];
    let mut stack = vec![0];
    label[0] = 0;
    while let Some(u) = stack.pop() {
        let l = label[u];
        for &v in &adj_list[u] {
            if label[v] != usize::MAX {
                continue;
            }
            label[v] = l ^ 1;
            stack.push(v);
        }
    }
    // eprintln!("{:?}", label);
    let count = label.iter().filter(|&&li| li == 0).count();
    if count >= n / 2 {
        label.iter().enumerate().filter(|&(_, &li)| li == 0).take(n / 2).for_each(|(i, _)| print!("{} ", i + 1));
    } else {
        label.iter().enumerate().filter(|&(_, &li)| li == 1).take(n / 2).for_each(|(i, _)| print!("{} ", i + 1));
    }
}
