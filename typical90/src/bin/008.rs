use proconio::{input, marker};
use util::modulo::ModInt;
type Mint = ModInt<1000000007>;

fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
    }
    let t = b"atcoder";
    let mut dp = vec![[Mint::new(0); 8]; n + 1];
    for row in &mut dp {
        row[0] = Mint::new(1);
    }
    for i in 1..=n {
        for j in 1..8 {
            dp[i][j] = dp[i - 1][j];
            if s[i - 1] == t[j - 1] {
                let tmp = dp[i - 1][j - 1];
                dp[i][j] += tmp;
            }
        }
    }
    println!("{}", dp[n][7]);
}
