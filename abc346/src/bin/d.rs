use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
        c: [u64; n],
    }
    // dp[i][j][k]: s[..i]までを考えたときに、操作後の文字列の隣り合う2文字が一致するものの個数がjであり、最後の文字がkであるときの最小コスト
    let mut dp = vec![[[u64::MAX, u64::MAX], [u64::MAX, u64::MAX]]; n];
    dp[0][0] = [0, 0];
    match s[0] {
        b'0' => dp[0][0][1] = c[0],
        b'1' => dp[0][0][0] = c[0],
        _ => unreachable!(),
    }
    for i in 0..n - 1 {
        match s[i + 1] {
            b'0' => {
                dp[i + 1][0][1] = dp[i + 1][0][1].min(dp[i][0][0].saturating_add(c[i + 1]));
                dp[i + 1][1][0] = dp[i + 1][1][0].min(dp[i][0][0]);
                dp[i + 1][0][0] = dp[i + 1][0][0].min(dp[i][0][1]);
                dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][0][1].saturating_add(c[i + 1]));
                dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][1][0].saturating_add(c[i + 1]));
                dp[i + 1][1][0] = dp[i + 1][1][0].min(dp[i][1][1]);
            }
            b'1' => {
                dp[i + 1][0][1] = dp[i + 1][0][1].min(dp[i][0][0]);
                dp[i + 1][1][0] = dp[i + 1][1][0].min(dp[i][0][0].saturating_add(c[i + 1]));
                dp[i + 1][0][0] = dp[i + 1][0][0].min(dp[i][0][1].saturating_add(c[i + 1]));
                dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][0][1]);
                dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][1][0]);
                dp[i + 1][1][0] = dp[i + 1][1][0].min(dp[i][1][1].saturating_add(c[i + 1]));
            }
            _ => unreachable!(),
        }
    }
    let ans = *dp[n - 1][1].iter().min().unwrap();
    println!("{}", ans);
}
