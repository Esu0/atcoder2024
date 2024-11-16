use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: marker::Bytes,
    }
    let mut x = 0;
    let mut j = usize::MAX;
    let mut prev = b'0';
    for (i, &ci) in s.iter().enumerate() {
        if prev == b'0' && ci == b'1' {
            x += 1;
            if k == x {
                j = i;
                break;
            }
        }
        prev = ci;
    }
    assert_ne!(j, usize::MAX);
    let mut i = j - 1;
    while s[i] == b'0' {
        i -= 1;
    }

    while j < n && s[j] == b'1' {
        i += 1;
        s.swap(i, j);
        j += 1;
    }
    println!("{}", std::str::from_utf8(&s).unwrap());
}
