use proconio::input;
use util::IteratorExt;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    a.sort_unstable();
    // count[i]: aに含まれるiの個数
    let mut count = vec![0u32; 1000001];
    for &ai in &a {
        count[ai as usize] += 1;
    }
    // count_sum[i]: aに含まれるi以下の数の個数
    let count_sum = count.iter().cumulative_sum(0, |&x, &y| x + y).skip(1).collect::<Vec<_>>();
    // eprintln!("{:?}", count);
    let mut ans = 0u64;
    for (i, &ci) in count.iter().enumerate().skip(1) {
        if ci != 0 {
            let ci = ci as u64;
            ans += ci * (ci - 1) / 2;
            let mut l = i;
            let mut r = i * 2 - 1;
            let mut k = 1;
            while l < 1000000 {
                let c = (count_sum[r.min(1000000)] - count_sum[l]) as u64;
                ans += ci * c * k;
                k += 1;
                l = r;
                r += i;
            }
        }
    }
    println!("{}", ans);
}
