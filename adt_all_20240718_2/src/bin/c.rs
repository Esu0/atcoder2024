use proconio::input;

fn main() {
    input! {
        s: String,
    }
    if "oxx".repeat(100).contains(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
