use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    // dp[i][j]: モンスターを倒す回数を2で割った余りがjになるようにi番目までのモンスターに対する行動を選んだときの得られる経験値の最大値
    let mut dpi = [0, a[0]];
    let mut dpi_next;
    for &ai in &a[1..] {
        dpi_next = dpi;
        for j in 0..2 {
            dpi_next[(j + 1) % 2] = dpi_next[(j + 1) % 2].max(dpi[j] + (j + 1) as u64 * ai);
        }
        dpi = dpi_next;
    }
    println!("{}", dpi.iter().copied().max().unwrap());
}
