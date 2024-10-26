use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(usize, usize); n],
    }
    lr.sort_unstable_by_key(|&(l, r)| (l, r));
    let mut range_set = lr.iter().map(|&(l, r)| (r, l)).collect::<BTreeSet<_>>();
    let mut ans = 0;
    let mut j = 0;
    for i in 1..=m {
        while j < n && lr[j].0 < i {
            range_set.remove(&(lr[j].1, lr[j].0));
            j += 1;
        }
        let r_min = range_set.first().map_or(m + 1, |&(r, _)| r);
        ans += r_min - i;
    }
    println!("{}", ans);
}
