use proconio::input;

fn main() {
    input! {
        mut n: u64,
        x: u64,
        y: u64,
    }
    let (mut r, mut b) = (1, 0);
    while n > 1 {
        n -= 1;
        (r, b) = (r + (r * x + b), (r * x + b) * y);
    }
    println!("{}", b);
}
