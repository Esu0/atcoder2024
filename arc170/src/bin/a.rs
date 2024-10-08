use proconio::{input, marker};

fn main() {
    input! {
        _n: usize,
        s: marker::Bytes,
        t: marker::Bytes,
    }
    if s == t {
        println!("0");
        return;
    }
    let mut l = 0;
    while l < s.len() && t[l] == b'B' {
        if s[l] == b'A' {
            println!("-1");
            return;
        }
        l += 1;
    }
    let mut r = s.len();
    while r > 0 && t[r - 1] == b'A' {
        r -= 1;
        if s[r] == b'B' {
            print!("-1");
            return;
        }
    }
    let s = &s[l..r];
    let t = &t[l..r];
    let n = s.len();
    assert_eq!(t[0], b'A');
    assert_eq!(*t.last().unwrap(), b'B');
    assert!(n >= 2);

    let mut i = 0;
    let mut j = 0;
    let mut count = 0;
    loop {
        while i < n && !(t[i] == b'A' && s[i] == b'B') {
            i += 1;
        }
        while j < n && !(t[j] == b'B' && s[j] == b'A') {
            j += 1;
        }
        if i >= n && j >= n {
            break;
        }
        if j >= n {
            count += 1;
            i += 1;
        } else if i < j {
            count += 1;
            i += 1;
            j += 1;
        } else {
            count += 1;
            j += 1;
        }
    }
    println!("{}", count);
}
