use proconio::{input, marker};

fn main() {
    input! {
        mut s: marker::Bytes,
    }
    s.iter_mut().for_each(|c| *c -= b'A');
    let mut pos = [0; 26];
    for (i, &c) in s.iter().enumerate() {
        pos[c as usize] = i;
    }
    let mut ans = 0;
    for w in pos.windows(2) {
        ans += w[0].abs_diff(w[1]);
    }
    println!("{}", ans);
}
