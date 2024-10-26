use proconio::input;

macro_rules! chmin {
    ($d:expr, $s:expr) => {
        $d = std::cmp::min($d, $s);
    };
}

fn main() {
    input! {
        n: usize,
        ab: [(u8, u32); n],
    }
    let s = ab.iter().map(|&(_, b)| b).sum::<u32>() as usize;
    if s % 3 != 0 {
        println!("-1");
        return;
    }
    let s = s / 3;
    let mut dpi = vec![vec![u8::MAX; s + 1]; s + 1];
    dpi[0][0] = 0;
    let mut dpi_next = vec![vec![u8::MAX; s + 1]; s + 1];
    for &(ai, bi) in ab.iter() {
        for j in 0..=s {
            for k in 0..=s {
                let dpijk = dpi[j][k];
                if dpijk == u8::MAX {
                    continue;
                }
                let nj = j + bi as usize;
                if nj <= s {
                    if ai == 1 {
                        chmin!(dpi_next[nj][k], dpijk);
                    } else {
                        chmin!(dpi_next[nj][k], dpijk + 1);
                    }
                }
                let nk = k + bi as usize;
                if nk <= s {
                    if ai == 2 {
                        chmin!(dpi_next[j][nk], dpijk);
                    } else {
                        chmin!(dpi_next[j][nk], dpijk + 1);
                    }
                }
                if ai == 3 {
                    chmin!(dpi_next[j][k], dpijk);
                } else {
                    chmin!(dpi_next[j][k], dpijk + 1);
                }
            }
        }
        std::mem::swap(&mut dpi, &mut dpi_next);
        dpi_next.iter_mut().for_each(|v| v.fill(u8::MAX));
    }
    let ans = dpi[s][s];
    println!("{}", ans as i8);
}
