use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
    }
    let mut ans = 0;
    for i in 0..n.saturating_sub(2) {
        if &s[i..=i+2] == b"#.#" {
            ans += 1;
        }
    }
    println!("{}", ans);
}
