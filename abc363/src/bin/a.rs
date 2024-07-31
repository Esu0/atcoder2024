use proconio::input;

fn main() {
    input! {
        r: u32,
    }
    match r {
        1..=99 => println!("{}", 100 - r),
        100..=199 => println!("{}", 200 - r),
        200..=299 => println!("{}", 300 - r),
        _ => unreachable!(),
    };
}
