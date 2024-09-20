use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i32; n],
        p: [u64; n],
        q: usize,
    }

    let mut xp = x.iter().copied().zip(p.iter().copied()).collect::<Vec<_>>();
    xp.sort_unstable_by_key(|&(x, _)| x);

    let mut ps = std::iter::once(0).chain(xp.iter().map(|&(_, p)| p)).collect::<Vec<_>>();
    for i in 1..n + 1 {
        ps[i] += ps[i - 1];
    }
    for _ in 0..q {
        input! { l: i32, r: i32 }
        let li = xp.partition_point(|&(x, _)| x < l);
        let ri = xp.partition_point(|&(x, _)| x <= r);
        // eprintln!("{} {}", li, ri);
        println!("{}", ps[ri] - ps[li]);
    }
}
