use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        x: [u64; 12],
    }
    let ans = [
        a * x.iter().map(|&xi| xi.saturating_sub(3)).sum::<u64>(),
        b + a * x.iter().map(|&xi| xi.saturating_sub(50)).sum::<u64>(),
        c,
    ]
    .iter()
    .copied()
    .min()
    .unwrap();
    println!("{}", ans);
}
