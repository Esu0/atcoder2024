use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
        b: [u64; n],
    }
    let lower = a.iter().zip(&b).map(|(&a, &b)| ((b + m - a) % m).min((a + m - b) % m)).sum::<u64>();
    eprintln!("{}", lower);
}
