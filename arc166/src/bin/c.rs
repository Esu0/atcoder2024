use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;

macro_rules! chsum {
    ($d:expr, $s:expr) => {
        {
            let tmp = $d + $s;
            $d = tmp;
        }
    };
}

fn main() {
    input! {
        t: usize,
    }
    let max_h = 1000000;
    let mut dp = vec![MInt::new(0); 2 * max_h + 2];
    let mut dp2 = vec![MInt::new(0); 2 * max_h + 2];
    dp[0] = MInt::new(1);
    dp[1] = MInt::new(1);
    for i in 2..dp.len() {
        chsum!(dp[i], dp2[i - 1]);
        chsum!(dp[i], dp[i - 1]);
        chsum!(dp2[i], dp[i - 1]);
    }
    for (dpi, &dp2i) in dp.iter_mut().zip(&dp2) {
        *dpi += dp2i;
    }
    // eprintln!("{:?}", &dp[0..10]);
    // prod[i]: dp[0] * dp[2] * ... * dp[2 * i]
    let mut prod = dp.iter().copied().step_by(2).collect::<Vec<_>>();
    for i in 1..prod.len() {
        let tmp = prod[i] * prod[i - 1];
        prod[i] = tmp;
    }
    for _ in 0..t {
        input! {
            mut h: usize,
            mut w: usize,
        }
        if h > w {
            std::mem::swap(&mut h, &mut w);
        }
        let ans = prod[h] * prod[h] * dp[2 * h + 1].pow((w - h) as u32);
        println!("{}", ans);
    }
}
