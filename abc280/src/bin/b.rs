use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [i64; n],
    }
    
    for i in (0..n - 1).rev() {
        s[i + 1] -= s[i];
    }
    print!("{}", s[0]);
    for &si in &s[1..] {
        print!(" {}", si);
    }
    println!();
}
