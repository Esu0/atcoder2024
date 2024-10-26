use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i32, i32); n],
    }
    let (mut x, mut y) = (0, 0);
    let mut ans = 0.;
    xy.push((0, 0));
    for (xi, yi) in xy {
        ans += ((x - xi) as f64).hypot((y - yi) as f64);
        (x, y) = (xi, yi);
    }
    println!("{}", ans);
}
