use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        mut d: usize,
        mut s: marker::Bytes,
    }
    let mut i = n;
    while d > 0 {
        d -= 1;
        while i > 0 && s[i - 1] != b'@' {
            i -= 1;
        }
        s[i - 1] = b'.';
    }
    println!("{}", std::str::from_utf8(&s).unwrap());
}
