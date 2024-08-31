use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let mut ans = vec![];
    let mut i = 0;
    while s[i] != b'|' {
        ans.push(s[i]);
        i += 1;
    }
    i += 1;
    while s[i] != b'|' {
        i += 1;
    }
    i += 1;
    ans.extend(s[i..].iter().copied());
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
