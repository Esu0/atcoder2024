use proconio::{input, marker};

fn main() {
    input! {
        mut s: marker::Bytes,
    }
    s.sort_unstable();

    if &s[..] == b"122333" {
        println!("Yes");
    } else {
        println!("No")
    }
}
