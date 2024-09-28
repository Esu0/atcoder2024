use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    for &(x, y) in &xy {
        let (ans, _) = xy.iter().enumerate().max_by_key(|&(i, &(x2, y2))| {
            let dx = x2 - x;
            let dy = y2 - y;
            let d = dx * dx + dy * dy;
            (d, Reverse(i))
        }).unwrap();
        println!("{}", ans + 1);
    }
}
