use proconio::input;

macro_rules! chmin {
    ($d:expr, $s:expr) => {
        $d = $d.min($s);
    };
}

fn main() {
    input! {
        n: usize,
        x: u32,
        y: u32,
        ab: [(u32, u32); n],
    }
    let mut dpi = vec![vec![u32::MAX; x as usize + 1]; n + 1];
    let mut dp_next = dpi.clone();
    dpi[0][0] = 0;
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        dp_next.clone_from(&dpi);
        if ai <= x {
            for j in 0..=i {
                for k in 0..=(x - ai) as usize {
                    let tmp = dpi[j][k].saturating_add(bi);
                    if tmp <= y {
                        chmin!(dp_next[j + 1][k + ai as usize], tmp);
                    }
                }
            }
        }
        std::mem::swap(&mut dpi, &mut dp_next);
    }
    // eprintln!("{:?}", dpi);
    let ans = dpi.iter().enumerate().rev().find(|(_, dpij)| dpij.iter().any(|&d| d <= y)).unwrap().0;
    println!("{}", n.min(ans + 1));
}
