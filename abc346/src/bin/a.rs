use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    for w in a.windows(2) {
        print!("{} ", w[0] * w[1]);
    }
    println!()
}
