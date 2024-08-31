use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    // dp[i][j]: モンスターを倒す回数を2で割った余りがjになるようにi番目までのモンスターに対する行動を選んだときの得られる経験値の最大値
    let mut dp = vec![[0u64; 2]; n];

    dp[0][0] = 0;
    dp[0][1] = a[0];
    for i in 0..n - 1 {
        dp[i + 1][1] = dp[i + 1][1].max(dp[i][0] + a[i + 1]);
        dp[i + 1][0] = dp[i + 1][0].max(dp[i][0]);
        dp[i + 1][0] = dp[i + 1][0].max(dp[i][1] + 2 * a[i + 1]);
        dp[i + 1][1] = dp[i + 1][1].max(dp[i][1]);
    }
    println!("{}", dp[n - 1][0].max(dp[n - 1][1]));
}
