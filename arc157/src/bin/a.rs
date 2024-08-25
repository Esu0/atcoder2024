use proconio::input;

fn main() {
    input! {
        _n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    if b == 0 && c == 0 {
        if a == 0 || d == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if b.abs_diff(c) <= 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
