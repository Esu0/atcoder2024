use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a_orig: [u32; n],
        b_orig: [u32; n],
    }
    let mut a = a_orig.iter().copied().enumerate().collect::<Vec<_>>();
    // let mut b = b.iter().copied().enumerate().collect::<Vec<_>>();
    a.sort_by_key(|&(_, x)| Reverse(x));
    let mut ans = Vec::with_capacity(x + y + z);
    ans.extend(a[..x].iter().map(|&(i, _)| i + 1));
    let mut b = a[x..].iter().map(|&(i, _)| (i, b_orig[i])).collect::<Vec<_>>();
    b.sort_unstable_by_key(|&(i, x)| (Reverse(x), i));
    ans.extend(b[..y].iter().map(|&(i, _)| i + 1));
    let mut c = b[y..].iter().map(|&(i, _)| (i, a_orig[i] + b_orig[i])).collect::<Vec<_>>();
    c.sort_unstable_by_key(|&(i, x)| (Reverse(x), i));
    ans.extend(c[..z].iter().map(|&(i, _)| i + 1));
    ans.sort_unstable();
    for i in ans {
        println!("{}", i);
    }
}
