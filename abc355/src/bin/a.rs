use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    }
    match (a, b) {
        (a, b) if a == b => println!("-1"),
        (1, 2) | (2, 1) => println!("3"),
        (1, 3) | (3, 1) => println!("2"),
        (2, 3) | (3, 2) => println!("1"),
        _ => unreachable!(),
    }
}
