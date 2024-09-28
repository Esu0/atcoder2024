use proconio::input;
fn mpow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    input! {
        n: u64,
        m: u64,
    }
    let m2 = m * m;
    let ans = ((mpow(10, n, m2) + m2 - mpow(10, n, m)) / m) % m;
    println!("{}", ans);
}
