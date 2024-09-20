use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut a = Vec::with_capacity(n);
    for i in 0..n {
        input! {
            ai: [usize; i + 1],
        }
        a.push(ai);
    }
    let mut elem = 1;
    for j in 1..=n {
        if elem >= j {
            elem = a[elem - 1][j - 1];
        } else {
            elem = a[j - 1][elem - 1];
        }
    }
    println!("{}", elem);
}
