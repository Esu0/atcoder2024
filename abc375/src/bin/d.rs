use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let mut mset1 = [0u64; 26];
    let mut mset2 = [0u64; 26];
    for &c in &s {
        mset2[(c - b'A') as usize] += 1;
    }
    let mut ans = 0;
    for &c in &s {
        mset2[(c - b'A') as usize] -= 1;
        ans += mset1.iter().zip(mset2.iter()).map(|(&a, &b)| a * b).sum::<u64>();
        mset1[(c - b'A') as usize] += 1;
    }
    println!("{}", ans);
}
