use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(u64, u64); n],
    }

    ab.sort_unstable_by(|&(ai, bi), &(aj, bj)| ((ai - 1) * bj).cmp(&((aj - 1) * bi)));
    // eprintln!("{:?}", ab);

    // dp[i][j]: 最後に適用した関数がi番目で、j個の関数を適用したときの最大値
    let mut dp = vec![vec![0; n]; k + 1];
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        dp[1][i] = ai + bi;
    }
    for i in 2..=k {
        let mut max = *dp[i - 1][0..i - 1].iter().max().unwrap();
        for (j, &(aj, bj)) in ab.iter().enumerate().skip(i - 1) {
            dp[i][j] = max * aj + bj;
            max = max.max(dp[i - 1][j]);
        }
    }
    println!("{}", dp[k].iter().max().unwrap());
}
