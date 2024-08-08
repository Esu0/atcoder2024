use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }

    a.sort_unstable();
    let mut ans = 0;
    let mut sum_rev = vec![0; n];
    {
        let mut s = 0;
        for (i, &ai) in a.iter().enumerate().rev() {
            s += ai;
            sum_rev[i] = s;
        }
    }
    eprintln!("{:?}", sum_rev);
    for (i, &ai) in a[..n - 1].iter().enumerate() {
        let count = (n - i - 1) - a[i + 1..].partition_point(|&aj| ai + aj < 100_000_000);
        ans += ai * (n - i - 1) as u64 + sum_rev[i + 1] - 100_000_000 * count as u64;
    }
    println!("{ans}");
}
