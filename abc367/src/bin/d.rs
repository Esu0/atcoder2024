use proconio::input;
use util::IteratorExt;

fn main() {
    input! {
        n: usize,
        m: u64,
        mut a: [u64; n],
    }
    a.iter_mut().for_each(|ai| *ai %= m);
    let mut count = vec![0u64; m as usize];
    let mut s = 0;
    a[..n - 1].iter().cumulative_sum(0, |&a, &b| a + b).skip(1).for_each(|sum| {
        count[(sum % m) as usize] += 1;
        s = sum;
    });
    // eprintln!("{:?}", count);
    let all_sum = s + a[n - 1];
    let mut ans = 0;
    let mut offset = 0;
    let mut offset2 = (all_sum % m) as usize;
    let mut s = 0;
    for &ai in &a[..n - 1] {
        ans += count[offset];
        ans += count[offset2];
        // eprintln!("{count:?}");
        s = (s + ai) % m;
        count[s as usize] -= 1;
        offset = (offset + ai as usize) % m as usize;
        offset2 = (offset2 + ai as usize) % m as usize;
    }
    println!("{}", ans);
}
