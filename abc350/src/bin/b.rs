use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [usize; q],
    }

    let mut exists = vec![true; n];
    for &ti in &t {
        exists[ti - 1] ^= true;
    }
    let ans = exists.into_iter().map(|b| b as u32).sum::<u32>();
    println!("{ans}");
}
