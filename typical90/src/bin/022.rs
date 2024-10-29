use proconio::input;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    while b != 0 {
        let r = a % b;
        (a, b) = (b, r);
    }
    a
}

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    let g = gcd(a, gcd(b, c));
    let ans = a / g - 1 + b / g - 1 + c / g - 1;
    println!("{}", ans);
}
