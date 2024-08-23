use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let mut count = [0; 26];
    for &c in &s {
        count[(c - b'a') as usize] += 1;
    }
    let ans = count.iter().position(|&x| x == 1).map(|x| x as u8 + b'a');
    if let Some(ans) = ans {
        println!("{}", ans as char);
    } else {
        println!("-1");
    }
}
