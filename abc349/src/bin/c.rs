use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
        t: marker::Bytes,
    }
    let mut j = 0;
    let t = if t[2] == b'X' {
        &t[..2]
    } else {
        &t
    };
    for &c in &s {
        if c - b'a' == t[j] - b'A' {
            j += 1;
        }
        if j == t.len() {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
