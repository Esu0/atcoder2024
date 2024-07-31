use proconio::input;

fn mpow(base: u32, mut exp: u64, modulo: u32) -> u32 {
    let mut base = base as u64;
    let modulo = modulo as u64;
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exp >>= 1;
    }
    result as u32
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u64,
            m: u64,
            k: u64,
        }
        if n < k {
            println!("{}", mpow(2, n, 10));
        } else if m - k == 1 {
            // 2^m - 2^k = 2^kかつn >= k
            println!("0");
        } else {
            let r = (n - k) % (m - k);
            println!("{}", mpow(2, k + r, 10));
        }
    }
}
