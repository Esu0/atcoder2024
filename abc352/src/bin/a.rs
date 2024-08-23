use proconio::input;

fn main() {
    input! {
        _n: u32,
        x: u32,
        y: u32,
        z: u32,
    }
    if x > y {
        if (y..x).contains(&z) {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if (x + 1..=y).contains(&z) {
        println!("Yes");
    } else {
        println!("No");
    }
}
