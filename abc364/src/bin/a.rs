use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: [marker::Bytes; n],
    }
    for w in s[..n - 1].windows(2) {
        if w[0][1] == b'w' && w[1][1] == b'w' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
