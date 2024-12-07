use std::collections::VecDeque;

use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        mut s: [marker::Bytes; h],
    }

    let mut count = 0;
    let mut queue = VecDeque::new();
    for (i, row) in s.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if *cell == b'H' {
                queue.push_back((0, (i, j)));
            }
        }
    }
    while let Some((dep, (i, j))) = queue.pop_front() {
        // eprintln!("{i} {j} {dep} {queue:?}");
        if dep > d {
            break;
        }
        if s[i][j] == 0 {
            continue;
        }
        s[i][j] = 0;
        count += 1;
        for (dx, dy) in [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)] {
            let ni = i.wrapping_add(dx);
            let nj = j.wrapping_add(dy);
            if (0..h).contains(&ni) && (0..w).contains(&nj) && s[ni][nj] == b'.' {
                queue.push_back((dep + 1, (ni, nj)));
            }
        }
    }
    println!("{count}");
}
