use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }

    let mut count = [0; 26];
    for &c in s.iter() {
        count[(c - b'a') as usize] += 1;
    }
    let c = count.iter().position(|&c| c == 1).unwrap() as u8;
    for (i, &ci) in s.iter().enumerate() {
        if ci == c + b'a' {
            println!("{}", i + 1);
        }
    }
}
