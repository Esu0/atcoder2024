use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
    }
    let m = s.iter().map(|&c| (c - b'0') as u64).collect::<Vec<_>>();
    let mut ans = vec![0; n];
    let mut s1 = 0;
    // let mut s2 = 0;
    for i in 0..n {
        s1 += m[i] * (i + 1) as u64;
        ans[i] = s1;
    }
    for i in (1..n).rev() {
        ans[i - 1] += ans[i] / 10;
        ans[i] %= 10;
    }
    for &ai in &ans {
        print!("{}", ai);
    }
    println!();
}
