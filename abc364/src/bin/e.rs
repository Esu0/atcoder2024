use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        y: u32,
        ab: [(u32, u32); n],
    }
    let ans1 = {
        let mut dp = vec![vec![vec![u32::MAX; x as usize + 1]; n + 1]; n + 1];
        dp[0][0][0] = 0;
        for i in 0..n {
            for j in 0..n {
                for k in 0..=x as usize {
                    let dpijk = dp[i][j][k];
                    let tmp = dpijk.saturating_add(ab[i].1);
                    let nk = k + ab[i].0 as usize;
                    if nk <= x as usize {
                        let m = &mut dp[i + 1][j + 1][nk];
                        if tmp <= y && *m > tmp {
                            *m = tmp;
                        }
                    }
                    let tmp = dp[i][j][k];
                    let m = &mut dp[i + 1][j][k];
                    if *m > tmp {
                        *m = tmp;
                    }
                }
            }
        }
        // eprintln!("{:?}", dp[1]);
        dp[n].iter().enumerate().rev().find(|(_, v)| v.iter().any(|&v| v != u32::MAX)).unwrap().0
    };
    println!("{}", n.min(ans1 + 1));

}
