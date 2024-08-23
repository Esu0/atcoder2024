use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [marker::Bytes; h],
    }
    let to_ind = |i: usize, j: usize| i * w + j;
    // let to_ij = |ind: usize| (ind / w, ind % w);

    let dij = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];
    let mut adj_list = vec![vec![]; w * h];
    for (i, si) in s.iter().enumerate() {
        for (j, &c) in si.iter().enumerate() {
            for &(di, dj) in &dij {
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                if ni < h && nj < w {
                    let c2 = s[ni][nj];
                    match (c, c2) {
                        (b's', b'n') | (b'n', b'u') | (b'u', b'k') | (b'k', b'e') | (b'e', b's') => {
                            adj_list[to_ind(i, j)].push(to_ind(ni, nj));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let mut visited = vec![false; w * h];
    let mut stack = vec![0];
    while let Some(v) = stack.pop() {
        if visited[v] {
            continue;
        }
        if v == w * h - 1 {
            println!("Yes");
            return;
        }
        visited[v] = true;
        for &u in &adj_list[v] {
            stack.push(u);
        }
    }
    println!("No");
}
