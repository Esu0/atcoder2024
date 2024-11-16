use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut dp = vec![vec![vec![false; n * n + 1]; n + 1]; n + 1];
    dp[0][0][0] = true;
    for i in 2..=n {
        for j in 2..=n {
            dp[i][j][i * j] = true;
            for l in 2..i - 1 {
                for m in 2..j - 1 {
                    let d = l * m;
                    for k in d..=n * n {
                        dp[i][j][k] |= dp[i - l][j - m][k - d];
                    }
                }
            }
        }
    }
    for i in 1..=n {
        for j in 1..=n {
            for l in 0..=i {
                for m in 0..=j {
                    for k in 0..=n * n {
                        dp[i][j][k] |= dp[l][m][k];
                    }
                }
            }
        }
    }
    for _ in 0..q {
        input! { k: usize }
        if dp[n][n][n * n - k] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
