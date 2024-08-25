use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }
    let mut t = 0;
    for &hi in &h {
        let mut hi = hi;
        t += hi / 5 * 3;
        hi %= 5;
        while hi > 0 {
            t += 1;
            if t % 3 == 0 {
                hi = hi.saturating_sub(3);
            } else {
                hi -= 1;
            }
        }
    }
    println!("{}", t);
}
