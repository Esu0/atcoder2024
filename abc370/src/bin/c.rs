use proconio::{input, marker};

fn main() {
    input! {
        mut s: marker::Bytes,
        t: marker::Bytes,
    }
    let n = s.len();
    let mut ans = vec![];
    for i in 0..n {
        if s[i] > t[i] {
            s[i] = t[i];
            ans.push(s.clone());
        }
    }
    for i in (0..n).rev() {
        if s[i] != t[i] {
            s[i] = t[i];
            ans.push(s.clone());
        }
    }
    println!("{}", ans.len());
    for a in &ans {
        println!("{}", std::str::from_utf8(a).unwrap());
    }
}
