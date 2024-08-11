use proconio::input;
use util::ModInt;
type MInt = ModInt<1000000007>;
fn main() {
    input! {
        n: u64,
        b: usize,
        k: usize,
        c: [u8; k],
    }
    let mut dp = vec![vec![MInt::new(0); b]; 61];
    let t = n.ilog2() as usize;
    for &ci in &c {
        dp[0][ci as usize % b] += MInt::new(1);
    }
    let mut pow10 = 10 % b;
    for i in 1..=t {
        for j1 in 0..b {
            for j2 in 0..b {
                let a = dp[i - 1][j1] * dp[i - 1][j2];
                dp[i][(pow10 * j1 + j2) % b] += a;
            }
        }
        pow10 = pow10 * pow10 % b;
    }

    let mut n = n;
    let mut t = 0;
    let mut pow10 = 10;
    let mut ans = vec![MInt::new(0); b];
    ans[0] = MInt::new(1);
    while n > 0 {
        if n & 1 == 1 {
            let mut new_ans = vec![MInt::new(0); b];
            for j1 in 0..b {
                for j2 in 0..b {
                    let a = ans[j1] * dp[t][j2];
                    new_ans[(j1 * pow10 + j2) % b] += a;
                }
            }
            ans = new_ans;
        }
        pow10 = pow10 * pow10 % b;
        n >>= 1;
        t += 1;
    }
    println!("{}", ans[0]);
}
