use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }
    let s = "wbwbwwbwbwbw".repeat(300).into_bytes();
    let l = w + b;
    let mut w_count = s[..l].iter().filter(|&&c| c == b'w').count();
    let mut b_count = s[..l].iter().filter(|&&c| c == b'b').count();
    for (&c1, &c2) in s[l..].iter().zip(&s) {
        // eprintln!("{} {}", w_count, b_count);
        if w_count == w && b_count == b {
            println!("Yes");
            return;
        }
        if c1 == b'w' {
            w_count += 1;
        } else {
            b_count += 1;
        }
        if c2 == b'w' {
            w_count -= 1;
        } else {
            b_count -= 1;
        }
    }
    println!("No");
}
