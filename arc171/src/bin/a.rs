use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u64,
            a: u64,
            b: u64,
        }
        if a > n {
            println!("No");
            continue;
        }
        let c = n - a;
        let r = c.min((n + 1) / 2);
        if r * c >= b {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
