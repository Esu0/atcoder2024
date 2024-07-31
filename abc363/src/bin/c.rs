use proconio::{input, marker};

fn search(n: usize, i: usize, buf: &mut [u8], count: &mut [usize; 26], f: &mut impl FnMut(&[u8])) {
    if i == n {
        f(buf);
        return;
    }
    for j in 0..26 {
        if count[j] == 0 {
            continue;
        }
        count[j] -= 1;
        buf[i] = b'a' + j as u8;
        search(n, i + 1, buf, count, f);
        count[j] += 1;
    }
}

fn is_palindrome(s: &[u8]) -> bool {
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: marker::Bytes,
    }
    let mut count = [0; 26];
    for &c in &s {
        count[(c - b'a') as usize] += 1;
    }
    // if count.iter().all(|&c| c <= 1) {
    //     let mut f = 1;
    //     for i in 1..=n {
    //         f *= i;
    //     }
    //     println!("{}", f);
    // }
    let mut buf = [0; 20];
    let mut ans = 0u32;
    search(n, 0, &mut buf[..n], &mut count, &mut |s| {
        for i in 0..n - k + 1 {
            if is_palindrome(&s[i..i + k]) {
                return;
            }
        }
        ans += 1;
    });
    println!("{ans}");
}
