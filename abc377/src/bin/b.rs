use proconio::{input, marker};

fn main() {
    input! {
        s: [marker::Bytes; 8],
    }
    let mut t = vec![vec![true; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == b'#' {
                for k in 0..8 {
                    t[k][j] = false;
                    t[i][k] = false;
                }
            }
        }
    }
    let ans = t
        .iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum::<usize>();
    println!("{}", ans);
}
