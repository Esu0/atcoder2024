use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    for ans in 0..10 {
        if ans != a + b {
            println!("{ans}");
            break;
        }
    }
}
