use proconio::input;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn main() {
    input! {
        n: u64,
        m: u64,
        k: u64,
    }
    let ans = util::upper_bound(0..4_000_000_000_000_000_000u64, |x| {
        let a = x / n;
        let b = x / m;
        let c = x / (n * m / gcd(n, m));
        a + b - 2 * c < k
    });
    assert!((ans % n == 0) ^ (ans % m == 0));
    println!("{}", ans);
}
