use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut dp = vec![vec![u64::MAX; n + 1]; n + 1];
    for (i, dpi) in dp[..n].iter_mut().enumerate() {
        dpi[i + 1] = 0;
    }
    // s[i] = a[..i].iter().sum()
    let mut s = vec![0; n + 1];
    s[1..].copy_from_slice(&a);
    for i in 1..=n {
        s[i] += s[i - 1];
    }

    for d in 2..=n {
        for l in 0..=n - d {
            let r = l + d;
            for m in l + 1..r {
                let tmp = dp[l][r].min(dp[l][m] + dp[m][r] + s[r] - s[l]);
                dp[l][r] = tmp;
            }
        }
    }
    println!("{}", dp[0][n]);
}
