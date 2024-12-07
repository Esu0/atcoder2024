use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(usize, usize); n],
    }
    let mut w = 0;
    let mut time = 0;
    for &(ti, vi) in &tv {
        while time < ti && w > 0 {
            w -= 1;
            time += 1;
        }
        time = ti;
        w += vi;
    }
    println!("{w}");
}
