use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u32,
        c: u64,
    }
    if c.checked_pow(b).is_some_and(|x| x <= a) {
        println!("No");
    } else {
        println!("Yes");
    }
}
