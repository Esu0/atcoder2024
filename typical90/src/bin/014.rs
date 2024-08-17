use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        mut b: [u64; n],
    }
    a.sort_unstable();
    b.sort_unstable();
    let ans = a.iter().zip(b.iter()).map(|(&ai, &bi)| ai.abs_diff(bi)).sum::<u64>();
    println!("{}", ans);
}
