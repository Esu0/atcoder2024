use proconio::{input, marker};

fn main() {
    input! {
        n: marker::Bytes,
    }

    let ans1 = vec![n[1], n[2], n[0]];
    let ans2 = vec![n[2], n[0], n[1]];
    println!("{} {}", std::str::from_utf8(&ans1).unwrap(), std::str::from_utf8(&ans2).unwrap());
}
