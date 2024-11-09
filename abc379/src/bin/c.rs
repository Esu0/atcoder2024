use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; m],
        a: [usize; m],
    }
    let mut xa = x.iter().copied().zip(a.iter().copied()).collect::<Vec<_>>();
    xa.sort_unstable_by_key(|&(x, _)| x);
    if xa[0].0 != 1 {
        println!("-1");
        return;
    }
    let mut ans = 0;
    let mut cur = 1;
    let mut stones = xa[0].1;
    for &(xi, ai) in xa.iter().skip(1) {
        let d = xi - cur;
        if stones < d {
            println!("-1");
            return;
        }
        ans += stones * d - d * (d + 1) / 2;
        stones -= d;
        stones += ai;
        cur = xi;
    }
    if stones != n - cur + 1 {
        println!("-1");
    } else {
        ans += stones * (stones - 1) / 2;
        println!("{}", ans);
    }
}
