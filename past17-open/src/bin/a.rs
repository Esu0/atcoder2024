use proconio::input;

fn main() {
    input! {
        h: u32,
        w: u32,
    }
    let h2 = h * h;
    if 10000 * w < h2 * 20 {
        println!("A");
    } else if 10000 * w < h2 * 25 {
        println!("B");
    } else {
        println!("C");
    }
}
