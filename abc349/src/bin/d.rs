use proconio::input;

fn solve(a: u64, b: u64, l: u64, r: u64, f: &mut impl FnMut(u64, u64)) {
    if r <= a || b <= l {
        return;
    }
    if l <= a && b <= r {
        f(a, b);
        return;
    }
    let m = (a + b) / 2;
    solve(a, m, l, r, f);
    solve(m, b, l, r, f);
}

fn main() {
    input! {
        l: u64,
        r: u64,
    }

    let mut v = Vec::new();
    solve(0, 1 << 60, l, r, &mut |a, b| {
        v.push((a, b));
    });
    v.sort_unstable_by_key(|&(a, _)| a);
    println!("{}", v.len());
    for (a, b) in v {
        println!("{} {}", a, b);
    }
}
