use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        a: [u32; n],
    }
    let ans = a.iter().copied().filter(|&ai| ai % k == 0).map(|ai| ai / k);
    ans.for_each(|ai| print!("{} ", ai));
    println!();
}
