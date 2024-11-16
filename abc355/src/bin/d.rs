use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }
    lr.sort_unstable_by_key(|&(l, _)| l);
    let mut ans = 0;
    for (i, &(_, ri)) in lr.iter().enumerate() {
        let c = lr[i + 1..].partition_point(|&(lj, _)| lj <= ri);
        ans += c;
    }
    println!("{}", ans);
}
