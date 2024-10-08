use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }

    if s.get(s.len().wrapping_sub(3)..).is_some_and(|suf| suf == b"san") {
        println!("Yes");
    } else {
        println!("No");
    }
}
