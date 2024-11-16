use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
        q: usize,
    }

    for i in 0..q {
        input! { k: usize }
        let k = k - 1;
        let a = k / s.len();
        let offset = k % s.len();
        let popcount = a.count_ones();
        if popcount % 2 == 0 {
            print!("{}", s[offset] as char);
        } else if s[offset].is_ascii_uppercase() {
            print!("{}", s[offset].to_ascii_lowercase() as char);
        } else {
            print!("{}", s[offset].to_ascii_uppercase() as char);
        }
        if i == q - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
