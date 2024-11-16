use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }
    let mut pos = vec![usize::MAX; n];
    for (i, &pi) in p.iter().enumerate() {
        pos[pi - 1] = i;
    }

    let mut set = pos[..k].iter().copied().collect::<BTreeSet<_>>();
    let mut ans = *set.last().unwrap() - *set.first().unwrap();
    for i in k..n {
        set.remove(&pos[i - k]);
        set.insert(pos[i]);
        ans = ans.min(*set.last().unwrap() - *set.first().unwrap());
    }
    println!("{}", ans);
}
