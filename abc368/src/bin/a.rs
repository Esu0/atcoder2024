use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i32; n],
    }

    let ans = a[n - k..].iter().chain(a[..n - k].iter());
    for &x in ans {
        print!("{} ", x);
    }
    println!();
}
