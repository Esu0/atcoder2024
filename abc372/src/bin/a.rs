use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let ans = s.iter().copied().filter(|&c| c != b'.').collect::<Vec<_>>();
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
