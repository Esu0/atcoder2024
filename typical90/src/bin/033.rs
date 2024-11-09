use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    if h == 1 {
        println!("{}", w);
    } else if w == 1 {
        println!("{}", h);
    } else {
        println!("{}", ((h + 1) / 2) * ((w + 1) / 2))
    }
}
