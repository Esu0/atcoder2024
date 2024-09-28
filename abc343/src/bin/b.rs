use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u8; n]; n],
    }
    for row in &a {
        for (j, &x) in row.iter().enumerate() {
            if x == 1 {
                print!("{} ", j + 1);
            }
        }
        println!();
    }
}
