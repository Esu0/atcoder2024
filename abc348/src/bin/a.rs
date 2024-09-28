use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let s = "oox".repeat(34);
    println!("{}", &s[..n]);
}
