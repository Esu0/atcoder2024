use proconio::input;
use segtree::{operation, Segtree};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut c: [usize; n],
        lr: [(usize, usize); q],
    }
    c.iter_mut().for_each(|ci| *ci -= 1);
    let mut lri = lr.iter().copied().enumerate().collect::<Vec<_>>();
    lri.sort_unstable_by_key(|&(_, (l, _))| l);
    let mut pos = vec![vec![]; n];
    for (i, &ci) in c.iter().enumerate().rev() {
        pos[ci].push(i);
    }

    let mut segtree = c
        .iter()
        .enumerate()
        .map(|(i, &ci)| {
            if pos[ci].last().copied() == Some(i) {
                1
            } else {
                0
            }
        })
        .collect::<Segtree<u32, operation::Add<_>>>();
    pos.iter_mut().for_each(|p| {p.pop();});
    let mut prev_l = 0;
    let mut ans = vec![0; q];
    for &(i, (l, r)) in &lri {
        let l = l - 1;
        while prev_l < l {
            segtree.update(prev_l, 0);
            if let Some(i) = pos[c[prev_l]].pop() {
                segtree.update(i, 1);
            }
            prev_l += 1;
        }
        ans[i] = segtree.query(..r);
    }
    for a in ans {
        println!("{}", a);
    }
}
