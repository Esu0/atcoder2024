use proconio::{input, marker};

macro_rules! chmin {
    ($base:expr, $value:expr) => {
        if $base > $value {
            $base = $value;
        }
    };
}

fn main() {
    input! {
        t: marker::Bytes,
        n: usize,
        s: [[marker::Bytes]; n],
    }
    let mut dp = vec![vec![u32::MAX; t.len() + 1]; n + 1];
    dp[0][0] = 0;
    for (i, si) in s.iter().enumerate() {
        // let ai = si.len();
        for j in 0..=t.len() {
            dp[i + 1][j] = dp[i][j];
        }
        for j in 0..t.len() {
            let dpij = dp[i][j];
            if dpij != u32::MAX {
                for sik in si {
                    if j + sik.len() <= t.len() && t[j..j + sik.len()] == sik[..] {
                        chmin!(dp[i + 1][j + sik.len()], dpij + 1);
                    }
                }
            }
        }
    }
    let ans = dp[n][t.len()];
    if ans == u32::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
