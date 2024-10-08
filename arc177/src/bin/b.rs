use proconio::{input, marker};

fn main() {
    input! {
        _n: usize,
        s: marker::Bytes,
    }
    let mut ans = vec![];
    for (i, &si) in s.iter().enumerate().rev() {
        if si == b'1' {
            ans.extend((0..=i).map(|_| b'A'));
            ans.extend((0..i).map(|_| b'B'));
        }
    }
    println!("{}", ans.len());
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
