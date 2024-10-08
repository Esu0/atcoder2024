use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        p: [u64; 10],
    }
    let ans = a.iter().map(|&ai| p[ai - 1]).sum::<u64>();
    println!("{ans}");
}
