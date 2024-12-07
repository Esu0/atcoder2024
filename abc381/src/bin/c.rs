use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
    }
    let mut ans = 1;
    for (i, &ci) in s.iter().enumerate() {
        if ci != b'/' {
            continue;
        }
        let mut count = 1;
        while i + count < n && s[i + count] == b'2' && i >= count && s[i - count] == b'1' {
            count += 1;
        }
        ans = ans.max(count * 2 - 1);
    }
    println!("{ans}");
}
