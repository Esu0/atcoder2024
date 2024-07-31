use std::collections::HashSet;

use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [marker::Bytes; h],
    }

    let mut unvisited = vec![true; w * h];
    let to_node = |x: usize, y: usize| x + y * w;
    let to_xy = |n: usize| (n % w, n / w);

    let iter = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    let mut max_count = 1;
    let mut visited = HashSet::new();
    for n in 0..w * h {
        let (x, y) = to_xy(n);
        if !unvisited[n] {
            continue;
        }
        unvisited[n] = false;
        if iter
            .into_iter()
            .map(|(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy)))
            .any(|(x, y)| x < w && y < h && s[y][x] == b'#')
        {
            continue;
        }
        if s[y][x] == b'#' {
            continue;
        }
        let mut stack = vec![n];
        let mut count = 1u32;
        visited.clear();
        visited.insert(n);
        while let Some(node) = stack.pop() {
            let (x, y) = to_xy(node);
            let filtered = iter
                .into_iter()
                .map(|(dx, dy)| {
                    let nx = x.wrapping_add(dx);
                    let ny = y.wrapping_add(dy);
                    (nx, ny)
                })
                .filter(|&(nx, ny)| nx < w && ny < h);

            let mut tmp = filtered.clone();
            if tmp.any(|(x, y)| s[y][x] == b'#') {
                continue;
            }

            for (nx, ny) in filtered {
                let next = to_node(nx, ny);
                if visited.insert(next) {
                    unvisited[next] = false;
                    stack.push(next);
                    count += 1;
                }
            }
        }
        max_count = max_count.max(count);
    }

    println!("{max_count}");
}
