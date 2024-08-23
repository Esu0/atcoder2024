use proconio::input;

fn solve(i: usize, n: usize, r: &[u32], buf: &mut [u32], sum: u32, k: u32) {
    if i == n {
        if sum % k == 0 {
            for &c in &buf[..n] {
                print!("{} ", c);
            }
            println!();
        }
        return;
    }
    for j in 1..=r[i] {
        buf[i] = j;
        solve(i + 1, n, r, buf, sum + j, k);
    }
}
fn main() {
    input! {
        n: usize,
        k: u32,
        r: [u32; n],
    }
    let mut buf = [0; 8];
    solve(0, n, &r, &mut buf, 0, k);
}
