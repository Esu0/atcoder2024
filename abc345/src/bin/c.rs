use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let mut count = [0; 26];
    count[(s[0] - b'a') as usize] += 1;
    let mut flg = false;
    let mut ans = 0;
    for (i, &c) in s.iter().enumerate().skip(1) {
        if !flg && count[(c - b'a') as usize] != 0 {
            flg = true;
            ans += 1;
        }
        ans += i - count[(c - b'a') as usize];
        count[(c - b'a') as usize] += 1;
    }
    println!("{ans}");
}
