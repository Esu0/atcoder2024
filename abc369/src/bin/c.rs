use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    for i in 0..n - 1 {
        a[i] -= a[i + 1];
    }
    let mut ans = 0u64;
    let mut cnt = 0;
    let mut prev = 0;
    for &di in &a[..n - 1] {
        if prev == di {
            cnt += 1;
        } else {
            ans += (cnt + 1) as u64 * (cnt + 2) as u64 / 2 - 1;
            cnt = 1;
            prev = di;
        }
    }
    ans += (cnt + 1) as u64 * (cnt + 2) as u64 / 2;
    println!("{}", ans);
}
