use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
        }
        if m == 1 {
            println!("Bob");
            continue;
        }
        if (1..=n).contains(&(n * (n + 1) % m)) {
            println!("Bob");
        } else {
            println!("Alice");
        }
    }
}
