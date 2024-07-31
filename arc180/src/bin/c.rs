use proconio::input;

const MODULO: u64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [i8; n],
    }

    let m = n * 20 + 1;
    let offset = n * 10;
    // dp[i][j + offset]: i番目までを考えたときに累積和がjになる部分列を選んだ時のAの変化後の数列の数
    let mut dp = vec![vec![0u64; m]; n + 1];
    // dp2[i][j + offset]: i番目までを考えたときに累積和がjかつ末尾がjとなる部分列を選んだ時のAの変化後の数列の数
    let mut dp2 = vec![vec![0u64; m]; n + 1];
    dp[0][offset] = 1;
    for (i, &ai) in a.iter().enumerate() {
        for j in 0..m {
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= MODULO;
            dp2[i + 1][j] = dp2[i][j];
            // dp2[i + 1][j] %= MODULO;
            let k = (j as isize + a[i] as isize) as usize;
            if ai != 0 && k < m {
                if j == offset {
                    dp[i + 1][k] += dp[i][j] + MODULO - dp2[i][k];
                } else {
                    dp[i + 1][k] += dp[i][j];
                }
                dp[i + 1][k] %= MODULO;
                // println!("{} {} {}", i + 1, k, dp[i + 1][k]);
            }
        }
        if ai != 0 {
            dp2[i + 1][(ai as isize + offset as isize) as usize] = dp[i][offset];
        }
    }
    for row in &dp {
        eprintln!("{:?}", row[offset]);
    }
    // for row in &dp2 {
    //     eprintln!("{:?}", &row[offset - 1..]);
    // }
    eprintln!("{:?}", &dp2[n][offset - 1..offset + 2]);
    let ans = (dp[n].iter().sum::<u64>() % MODULO + MODULO - dp2[n].iter().sum::<u64>() % MODULO) % MODULO;
    println!("{ans}");
}
