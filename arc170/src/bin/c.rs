use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [u8; n],
    }
    let h = n.min(m + 1);
    // dp[i][j]: validな数列(A_0, A_1, ..., A_{i - 1})の中で、ちょうどj種類の数が使われているものの個数
    let mut dp = vec![vec![MInt::new(0); n + 1]; n + 1];
    dp[0][0] = MInt::new(1);
    for i in 1..=n {
        for j in 1..=h {
            if s[i - 1] == 0 {
                dp[i][j] = MInt::new(j as _) * dp[i - 1][j] + MInt::new((m + 1 - j) as _) * dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    let ans = dp[n][1..=h].iter().fold(MInt::new(0), |acc, &x| acc + x);
    println!("{}", ans);
}
