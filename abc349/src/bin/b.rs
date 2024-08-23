use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let mut count = [0u32; 26];
    for &c in &s {
        count[(c - b'a') as usize] += 1;
    }
    if (1..=100).all(|i| matches!(count.iter().filter(|&&c| c == i).count(), 0 | 2)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
