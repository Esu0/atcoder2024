use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
        t: marker::Bytes,
    }
    let mut i = 0;
    for (j, &c) in t.iter().enumerate() {
        if c == s[i] {
            i += 1;
            print!("{} ", j + 1);
        }
    }
    println!();
}
