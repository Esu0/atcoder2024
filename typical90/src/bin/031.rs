use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [usize; n],
        b: [usize; n],
    }
    let mut dp = std::array::from_fn::<_, 51, _>(|_| vec![u32::MAX; 1326]);
    dp[0][1] = 0;
    let mut mb = 1325;
    let mut buf = [false; 1000];
    for w in 0..=50 {
        mb -= w;
        for b in 0..=mb {
            buf.fill(false);
            if w > 0 {
                buf[dp[w - 1][b + w] as usize] = true;
            }
            for k in 1..=b / 2 {
                buf[dp[w][b - k] as usize] = true;
            }
            let mut mex = 0;
            while buf[mex] {
                mex += 1;
            }
            dp[w][b] = mex as u32;
        }
    }
    let mut g = 0;
    for (&wi, &bi) in w.iter().zip(&b) {
        g ^= dp[wi][bi];
    }
    if g == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
