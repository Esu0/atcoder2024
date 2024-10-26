use proconio::input;

macro_rules! chmin {
    ($d:expr, $s:expr) => {
        $d = std::cmp::min($d, $s);
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, usize); q],
    }
    let mut dp = vec![vec![usize::MAX; n]; q + 1];
    let (mut prev_h, mut prev_t) = ('L', 0);
    dp[0][1] = 0;

    let op = move |l_pos: usize, r_pos: usize, move_left: bool, pos: usize| {
        let (l_pos, r_pos) = if move_left {
            (l_pos, r_pos)
        } else {
            (r_pos, l_pos)
        };
        // 手を正の方向に動かす場合
        let res1 = {
            let r = if r_pos < l_pos { r_pos + n } else { r_pos };
            let p = if pos < l_pos { pos + n } else { pos };
            if r > p {
                (p - l_pos, r_pos)
            } else {
                // 左手はpの位置、右手はp + 1の位置に移動させる
                (p - l_pos + p + 1 - r, (p + 1) % n)
            }
        };
        // 手を負の方向に動かす場合
        let res2 = {
            let l = if l_pos < pos { l_pos + n } else { l_pos };
            let r = if r_pos < pos { r_pos + n } else { r_pos };
            if l < r {
                (l - pos, r_pos)
            } else {
                // 左手はposの位置、右手はpos - 1の位置に移動させる
                (l - pos + r - pos + 1, (pos + n - 1) % n)
            }
        };
        [res1, res2]
    };

    for i in 0..q {
        let (hi, ti) = ht[i];
        let ti = ti - 1;
        for j in 0..n {
            let dpij = dp[i][j];
            if dpij == usize::MAX {
                continue;
            }
            let x = if prev_h == 'L' {
                op(prev_t, j, hi == 'L', ti)
            } else {
                op(j, prev_t, hi == 'L', ti)
            };
            // eprintln!("{:?}", x);
            for &(d, nj) in &x {
                chmin!(dp[i + 1][nj], dpij + d);
            }
        }
        prev_h = hi;
        prev_t = ti;
    }
    println!("{}", dp[q].iter().copied().min().unwrap());
}
