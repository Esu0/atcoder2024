use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: marker::Bytes,
    }
    let mut map = vec![false; n];
    let mut ans = 0u32;
    for (i, mapi) in map[..n - 2].iter_mut().enumerate() {
        // eprintln!("{}", std::str::from_utf8(&s[i..i + 3]).unwrap());
        if &s[i..i + 3] == b"ABC" {
            *mapi = true;
            ans += 1;
        }
    }

    for _ in 0..q {
        input! {
            x: usize,
            c: char,
        }
        // eprintln!("{:?}", map);
        let x = x - 1;
        if s[x] == c as u8 {
            println!("{}", ans);
            continue;
        }
        s[x] = c as u8;
        if x >= 2 && map[x - 2] {
            map[x - 2] = false;
            ans -= 1;
        }
        if x >= 1 && map[x - 1] {
            map[x - 1] = false;
            ans -= 1;
        }
        if map[x] {
            map[x] = false;
            ans -= 1;
        }
        if x >= 2 && &s[x - 2..=x] == b"ABC" {
            map[x - 2] = true;
            ans += 1;
        }
        if x >= 1 && x + 1 < n && &s[x - 1..=x + 1] == b"ABC" {
            map[x - 1] = true;
            ans += 1;
        }
        if x + 3 <= n && &s[x..x + 3] == b"ABC" {
            map[x] = true;
            ans += 1;
        }
        println!("{}", ans);
    }
}
