use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
    }

    let k = n - m;
    a.sort_unstable();
    let ans = a.iter().map(|&ai| ai * ai).sum::<u64>()
        + a[0..k]
            .iter()
            .zip(a[k..k * 2].iter().rev())
            .map(|(&ai, &bi)| ai * bi * 2)
            .sum::<u64>();
    println!("{}", ans);
}
