use proconio::input;
use segtree::{operation, Segtree};

fn main() {
    input! {
        n: usize,
        mut h: [usize; n],
    }
    h.iter_mut().for_each(|x| *x -= 1);
    let mut ans = vec![0; n];
    let mut segtree = (0..n).map(|_| 0).collect::<Segtree<_, operation::Max<_>>>();
    for (j, &hj) in h.iter().enumerate().skip(1) {
        let hjp = h[j - 1];
        let mut re = segtree.get_mut(hjp);
        if *re < j - 1 {
            *re = j - 1;
        }
        drop(re);
        let l = segtree.query(hj + 1..);
        // dbg!(l);
        ans[l] += 1;
        ans[j] -= 1;
    }
    for i in 1..n {
        ans[i] += ans[i - 1];
    }
    // ans[..n - 1].iter_mut().for_each(|x| *x += 1);
    ans.iter().for_each(|&x| print!("{} ", x));
}
