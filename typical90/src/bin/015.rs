use proconio::input;
use util::ModInt;
type MInt = ModInt<1000000007>;

fn main() {
    input! {
        n: usize,
    }
    let mut factorial = vec![MInt::new(1); n + 1];
    for i in 1..=n {
        factorial[i] = factorial[i - 1] * MInt::new(i as _);
    }
    let factorial_inv = factorial.iter().map(|&x| x.inv()).collect::<Vec<_>>();
    let ans = (1..=n).map(|k| {
        let k = k - 1;
        let mut sum = MInt::new(0);
        let mut i = 0;
        while n > (k + 1) * i {
            let n = n - k * i;
            let r = i + 1;
            sum += factorial[n] * factorial_inv[r] * factorial_inv[n - r];
            i += 1;
        }
        sum
    }).collect::<Vec<_>>();
    for &a in &ans {
        println!("{}", a);
    }
}
