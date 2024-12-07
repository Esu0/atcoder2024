use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    if n % 2 == 0 {
        println!("No");
        return;
    }
    let t = "1".repeat(n / 2) + "/" + "2".repeat(n / 2).as_str();
    if s == t {
        println!("Yes");
    } else {
        println!("No");
    }
}
