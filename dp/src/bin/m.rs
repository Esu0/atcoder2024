use proconio::input;
use util::ModInt;
type MInt = ModInt<1000000007>;
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
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![MInt::new(0); k + 1]; n + 1];
    dp[0][0] = MInt::new(1);
    for i in 0..n {
        for j in 1..=k {
            chsum!(dp[i][j], dp[i][j - 1]);
        }
        for j in 0..=k {
            let tmp = dp[i][j] - j.checked_sub(a[i] + 1).map_or(MInt::new(0), |x| dp[i][x]);
            dp[i + 1][j] = tmp;
        }
    }
    println!("{}", dp[n][k]);
}
