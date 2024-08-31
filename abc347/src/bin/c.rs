use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u32,
        b: u32,
        mut d: [u32; n],
    }
    d.iter_mut().for_each(|x| *x %= a + b);
    d.sort_unstable();
    let mut max = d.windows(2).map(|x| x[1] - x[0]).max().unwrap_or_default();
    max = max.max(d[0] + a + b - d[n - 1]);
    if max > b {
        println!("Yes");
    } else {
        println!("No");
    }
}
