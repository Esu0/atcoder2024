use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        m: usize,
        lrx: [(usize, usize, usize); m],
    }

    let mut dp = vec![vec![MInt::new(0); n + 1]; n + 1];
    let mut dp2 = vec![vec![vec![true; n + 1]; n + 1]; n + 1];
    let mut combi = vec![vec![MInt::new(0); n + 1]; n + 1];
    for (i, dpi) in dp.iter_mut().enumerate() {
        dpi[i] = MInt::new(1);
    }
    for (i, dp2i) in dp2.iter_mut().enumerate() {
        for &(l, r, x) in &lrx {
            if i < l {
                dp2i[r][x - 1] = false;
            }
        }
        for j in i + 1..=n {
            for k in 0..=n {
                dp2i[j][k] &= dp2i[j - 1][k];
            }
        }
    }
    // dp2.iter().for_each(|dp2i| {
    //     eprintln!("{:?}", dp2i);
    // });
    // eprintln!("{:?}", dp2);

    combi[0][0] = MInt::new(1);
    for i in 1..=n {
        combi[i][0] = MInt::new(1);
        for j in 1..=i {
            combi[i][j] = combi[i - 1][j - 1] + combi[i - 1][j];
        }
    }

    // eprintln!("{:?}", combi);
    for d in 1..=n {
        for l in 0..=n - d {
            let r = l + d;
            for m in l..r {
                if dp2[l][r][m] {
                    let a = combi[d - 1][m - l] * dp[l][m] * dp[m + 1][r];
                    dp[l][r] += a;
                }
            }
        }
    }
    // dp.iter().for_each(|dpi| {
    //     eprintln!("{:?}", dpi);
    // });
    println!("{}", dp[0][n]);
}
