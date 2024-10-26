use proconio::input;

fn main() {
    input! {
        mut s: proconio::marker::Bytes,
    }
    s.sort_unstable();
    if s == b"ABC" {
        println!("Yes");
    } else {
        println!("No");
    }
}
