use proconio::{input, marker};

fn main() {
    input! {
        s: [marker::Bytes; 12],
    }
    let ans = s.iter().enumerate().filter(|&(i, s)| i + 1 == s.len()).count();
    println!("{}", ans);
}
