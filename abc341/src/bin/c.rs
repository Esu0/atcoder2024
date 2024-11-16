use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
        _n: usize,
        t: marker::Bytes,
        s: [marker::Bytes; h],
    }

    let mut ans = 0;
    for i in 0..h {
        'next: for j in 0..w {
            if s[i][j] == b'.' {
                let (mut ci, mut cj) = (i, j);
                for &d in &t {
                    match d {
                        b'U' => {
                            ci -= 1;
                        }
                        b'D' => {
                            ci += 1;
                        }
                        b'L' => {
                            cj -= 1;
                        }
                        b'R' => {
                            cj += 1;
                        }
                        _ => unreachable!(),
                    }
                    if s[ci][cj] == b'#' {
                        continue 'next;
                    }
                }
                ans += 1;
            }
        }
    }

    println!("{}", ans);

}
