use proconio::input;
use util::IteratorExt;

fn main() {
    input! {
        n: usize,
        cp: [(u8, u32); n],
        q: usize,
    }
    let cumsum = cp.iter().cumulative_sum([0, 0], |&[s1, s2], &(c, p)| {
        if c == 1 {
            [s1 + p, s2]
        } else {
            [s1, s2 + p]
        }
    }).collect::<Vec<_>>();
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        let ans1 = cumsum[r][0] - cumsum[l - 1][0];
        let ans2 = cumsum[r][1] - cumsum[l - 1][1];
        println!("{} {}", ans1, ans2);
    }
}
