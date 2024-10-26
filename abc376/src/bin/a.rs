use proconio::input;

fn main() {
    input! {
        n: usize,
        c: u32,
        t: [u32; n],
    }
    let mut prev = t[0];
    let mut ans = 1;
    for &ti in &t[1..] {
        if ti - prev >= c {
            ans += 1;
            prev = ti;
        }
    }
    println!("{}", ans);
}
