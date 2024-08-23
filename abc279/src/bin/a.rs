use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let ans = s.iter().map(|&c| if c == b'v' { 1 } else { 2 }).sum::<i32>();
    println!("{}", ans);
}
