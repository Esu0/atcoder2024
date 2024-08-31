use proconio::input;
use segtree::{operation, Segtree};

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        m: usize,
        a: [usize; m],
    }
    let mut segtree = Segtree::from_iter_op((0..n).map(|_| 1u32), operation::Add::default());
    let mut p_inv = vec![0; n];
    for i in 0..n {
        p_inv[p[i] - 1] = i;
    }
    let mut inv = vec![0; n];
    for &p in &p_inv {
        inv[p] = segtree.query(..p);
        *segtree.get_mut(p) = 0;
    }
    // eprintln!("{:?}", inv);
    let mut l = vec![0u32; n];
    for &ai in &a {
        if ai < n {
            l[ai] += 1;
        }
    }
    for i in 0..n - 1 {
        l[i + 1] += l[i];
    }
    eprintln!("{:?}", l);
    let mut ans = vec![0i64; m];
    for (&lx, &invx) in l.iter().zip(&inv) {
        let r = (lx + invx) as usize;
        let lx = lx as usize;
        if lx < m {
            ans[lx] += 1;
        }
        if r < m {
            ans[r] -= 1;
        }
    }
    // eprintln!("{:?}", ans);
    for i in 0..m - 1 {
        ans[i + 1] += ans[i];
    }
    let mut s = inv.iter().map(|&x| x as i64).sum::<i64>();
    for &ansi in &ans {
        s -= ansi;
        println!("{}", s);
    }
}
