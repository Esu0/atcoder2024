use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(usize, usize); n],
    }
    lr.sort_unstable_by_key(|&(_, r)| r);
    let mut l_max = 0;
    let mut j = 0;
    let mut ans = 0;
    for i in 1..=m {
        while j < n && lr[j].1 <= i {
            l_max = lr[j].0.max(l_max);
            j += 1;
        }
        ans += i - l_max;
    }
    println!("{}", ans);
}
