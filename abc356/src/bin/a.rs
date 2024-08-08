use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }
    let mut a = (1..=n).collect::<Vec<_>>();
    a[l - 1..r].reverse();
    a.iter().for_each(|&ai| print!("{ai} "));
    println!();
}
