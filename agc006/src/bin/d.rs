use proconio::input;
use util::algorithm::upper_bound;

fn main() {
    input! {
        n: usize,
        a: [usize; 2 * n - 1],
    }
    let mut buf = vec![false; 2 * n - 1];
    let ans = upper_bound(1..=2 * n - 1, |x| {
        // 頂点の数字がx以上のときtrue
        for (i, &ai) in a.iter().enumerate() {
            buf[i] = ai >= x;
        }
        let mut r = 0usize;
        let mut prev = buf[n - 1];
        for &bi in &buf[n..] {
            if bi != prev {
                r += 1;
                prev = bi;
            } else {
                break;
            }
        }
        let mut l = 0usize;
        prev = buf[n - 1];
        for &bi in buf[..n - 1].iter().rev() {
            if bi != prev {
                l += 1;
                prev = bi;
            } else {
                break;
            }
        }
        let mut ans = buf[n - 1];
        for _ in 0..l.min(r) {
            ans ^= true;
        }
        ans
    }) - 1;
    println!("{}", ans);
}
