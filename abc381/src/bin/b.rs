use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    if s.len() % 2 == 1 {
        println!("No");
        return;
    }
    let mut set = [false; 26];

    for i in 0..s.len() / 2 {
        if set[(s[2 * i] - b'a') as usize] || s[2 * i] != s[2 * i + 1] {
            println!("No");
            return;
        }
        set[(s[2 * i] - b'a') as usize] = true;
    }
    println!("Yes");
}
