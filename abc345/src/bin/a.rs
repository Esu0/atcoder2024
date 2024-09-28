use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let n = s.len();
    if *s.first().unwrap() == b'<' && *s.last().unwrap() == b'>' && s[1..n - 1].iter().all(|&c| c == b'=') {
        println!("Yes");
    } else {
        println!("No");
    }
}
