use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    }
    let ans = ab.iter().map(|&(a, _)| a).sum::<u64>() + ab.iter().map(|&(a, b)| b - a).max().unwrap();
    println!("{}", ans);
}
