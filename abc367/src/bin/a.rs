use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
        c: u8,
    }
    if b > c {
        if (c..b).contains(&a) {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if (b..c).contains(&a) {
        println!("No");
    } else {
        println!("Yes");
    }
}
