use proconio::input;

macro_rules! chmin {
    ($d:expr, $s:expr) => {
        $d = std::cmp::min($d, $s);
    };
}

fn main() {
    input! {
        n: usize,
        a: [u64; n * 2],
    }
    let n2 = n * 2;
    let mut dp = vec![vec![u64::MAX; n2 + 1]; n2 + 1];
    for (i, dpi) in dp.iter_mut().enumerate() {
        dpi[i] = 0;
    }

    for d in (0..=n2).step_by(2) {
        for l in 0..=n2 - d {
            let r = l + d;
            for k in (l + 1..r).step_by(2) {
                chmin!(dp[l][r], dp[l + 1][k] + dp[k + 1][r] + a[l].abs_diff(a[k]));
            }
        }
    }
    let ans = dp[0][n2];
    println!("{}", ans);
}
