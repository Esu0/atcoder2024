

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    if s.contains(&t) {
        println!("Yes");
    } else {
        println!("No");
    }
}
