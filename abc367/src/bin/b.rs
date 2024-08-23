use proconio::{input, marker};

fn main() {
    input! {
        mut x: marker::Bytes,
    }
    while x.last() == Some(&b'0') {
        x.pop();
    }
    if x.last() == Some(&b'.') {
        x.pop();
    }
    println!("{}", std::str::from_utf8(&x).unwrap());
}
