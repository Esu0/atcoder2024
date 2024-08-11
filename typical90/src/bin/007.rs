use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        q: usize,
        b: [u32; q],
    }
    a.sort_unstable();
    for &bi in &b {
        let i = a.partition_point(|&x| x <= bi);
        let ans = if i == 0 {
            a[i].abs_diff(bi)
        } else if i == n {
            a[i - 1].abs_diff(bi)
        } else {
            a[i - 1].abs_diff(bi).min(a[i].abs_diff(bi))
        };
        println!("{ans}");
    }
}
