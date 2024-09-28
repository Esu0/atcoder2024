use proconio::input;
macro_rules! chmin {
    ($base:expr, $cmp:expr) => {
        if $base > $cmp {
            $base = $cmp;
        }
    };
}

fn solve1() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut adj_mat = vec![vec![true; n]; n];
    for &(a, b) in &ab {
        adj_mat[a - 1][b - 1] = false;
        adj_mat[b - 1][a - 1] = false;
    }

    let mut dp = vec![u32::MAX; 1 << n];
    for bit in 1..1 << n {
        let mut flg = true;
        'outer: for i in 0..n {
            if bit & (1 << i) != 0 {
                for j in i + 1..n {
                    if bit & (1 << j) != 0 && adj_mat[i][j] {
                        flg = false;
                        break 'outer;
                    }
                }
            }
        }
        if flg {
            dp[bit] = 1;
        } else {
            let mut j = (bit - 1) & bit;
            while j > 0 {
                // eprintln!("{:02$b} {:02$b}", j, bit ^ j, n);
                chmin!(dp[bit], dp[j] + dp[bit ^ j]);
                j = (j - 1) & bit;
            }
        }
    }
    // eprintln!("{:?}", dp);
    println!("{}", dp[(1 << n) - 1]);
}

fn solve2() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut adj_mat = vec![0; n];
    for &(a, b) in &ab {
        adj_mat[a - 1] |= 1 << (b - 1);
        adj_mat[b - 1] |= 1 << (a - 1);
    }
    adj_mat.iter_mut().for_each(|x| *x ^= (1 << n) - 1);

    let mut ind = vec![u64::MAX; 1 << n];
    ind[0] = 0;
    for bit in 1..1usize << n {
        let i = bit.trailing_zeros();
        ind[bit] = ind[bit ^ (1 << i)] + ind[(bit ^ (1 << i)) & !adj_mat[i as usize]];
    }
}
fn main() {
    solve1();
}
