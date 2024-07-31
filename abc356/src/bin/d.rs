use proconio::input;

const MODULO: u64 = 998244353;

fn main() {
    input! {
        mut n: u64,
        m: u64,
    }
    n += 1;
    let mut sum = 0u64;
    let mut mask = 1u64;
    while mask <= m {
        if mask & m != 0 {
            sum += n / (mask * 2) % MODULO * mask % MODULO;
            let r = n % (mask * 2);
            sum += r.saturating_sub(mask) % MODULO;
        }
        mask <<= 1;
    }
    println!("{}", sum % MODULO);
}
