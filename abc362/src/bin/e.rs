use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut dp = vec![vec![vec![0; n]; n]; n + 1];

    // dp[i][j][k]: 長さi、最後がa[j]、最後から2番目がa[k]と等差数列の個数
    for j in 0..n {
        for k in 0..j {
            dp[2][j][k] = 1;
        }
    }

    for i in 3..=n {
        for j in 0..n {
            for k in 0..j {
                
            }
        }
    }
}
