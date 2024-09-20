use proconio::input;

fn main() {
    input! {
        l: u8,
        r: u8,
    }
    match (l, r) {
        (1, 0) => println!("Yes"),
        (0, 1) => println!("No"),
        _ => println!("Invalid"),
    }
}
