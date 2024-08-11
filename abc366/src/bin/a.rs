use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: usize,
    }

    if t > n / 2 || a > n / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
