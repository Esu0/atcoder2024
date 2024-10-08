use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let mut c = 0;
    let mut prev = 0;
    for &si in &s {
        if si == prev {
            c += 1;
        } else {
            if prev != 0 {
                print!("{} {} ", prev as char, c);
            }
            c = 1;
            prev = si;
        }
    }
    if prev != 0 {
        println!("{} {}", prev as char, c);
    }
}
