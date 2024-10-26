use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: usize,
        ab: [(i64, i64); m],
    }
    let mut set = HashSet::new();
    let dxy = [(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)];
    for &(a, b) in &ab {
        set.insert((a, b));
        for &(dx, dy) in &dxy {
            let nx = a + dx;
            let ny = b + dy;
            if (1..=n).contains(&nx) && (1..=n).contains(&ny) {
                set.insert((nx, ny));
            }
        }
    }
    let ans = n * n - set.len() as i64;
    println!("{}", ans);
}
