use proconio::{input, marker};
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
        s: marker::Bytes,
    }
    let n = s.len();
    let mut dp = vec![vec![MInt::new(0); n + 1]; n + 1];
    dp[0][0] = MInt::new(1);
    for i in 0..n {
        if s[i] == b'?' || s[i] == b'(' {
            for j in 0..=i {
                chsum!(dp[i + 1][j + 1], dp[i][j]);
            }
        }
        if s[i] == b'?' || s[i] == b')' {
            for j in 1..=i {
                chsum!(dp[i + 1][j - 1], dp[i][j]);
            }
        }
    }
    println!("{}", dp[n][0]);
}
