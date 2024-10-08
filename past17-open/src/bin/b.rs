use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: [marker::Bytes; n],
    }
    if s.iter().all(|s| s == b"Perfect") {
        println!("All Perfect");
    } else if s.iter().all(|s| s == b"Perfect" || s == b"Great") {
        println!("Full Combo");
    } else {
        println!("Failed");
    }
}
