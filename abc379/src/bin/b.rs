use proconio::{input, marker};

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: marker::Bytes,
    }

    let mut ans = 0;
    let mut count = 0;
    for &si in &s {
        if si == b'O' {
            count += 1;
            if count == k {
                ans += 1;
                count = 0;
            }
        } else {
            count = 0;
        }
    }
    println!("{}", ans);
}
