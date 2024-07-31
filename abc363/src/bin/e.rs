use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        y: u32,
        mut a: [[u32; w]; h],
    }
    let mut queue = BinaryHeap::new();
    for i in [0, h - 1] {
        for j in 0..w {
            let aij = a[i][j];
            if aij <= 100000 {
                queue.push((Reverse(a[i][j]), i, j));
                a[i][j] = u32::MAX;
            }
        }
    }

    for j in [0, w - 1] {
        for i in 0..h {
            let aij = a[i][j];
            if aij <= 100000 {
                queue.push((Reverse(a[i][j]), i, j));
                a[i][j] = u32::MAX;
            }
        }
    }

    let diff = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];
    let mut count = h * w;
    for k in 1..=y {
        while queue.peek().is_some_and(|&(Reverse(v), _, _)| v <= k) {
            let (_, i, j) = queue.pop().unwrap();
            count -= 1;
            for &(di, dj) in &diff {
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                if ni < h && nj < w && a[ni][nj] <= 100000 {
                    queue.push((Reverse(a[ni][nj]), ni, nj));
                    a[ni][nj] = u32::MAX;
                }
            }
        }
        println!("{}", count);
    }
}
