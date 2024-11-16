use std::collections::HashSet;

use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }
    let mut constraint = vec![usize::MAX; n];
    // let mut num_set = vec![true; n];
    let mut max_num = vec![n; n];
    let mut g = vec![vec![]; n];
    for &(a, b, c) in abc.iter() {
        if c == 1 {
            println!("0");
            return;
        }
        g[a - 1].push((b - 1, c - 1));
        g[b - 1].push((a - 1, c - 1));
    }
    g.iter_mut().for_each(|lst| lst.sort_unstable_by_key(|x| x.1));
    let mut del_set = HashSet::new();
    for a in 0..n {
        if g[a].len() <= 1 {
            continue;
        }
        let first = g[a][0].1;
        let i = if first == g[a][1].1 {
            if constraint[a] != usize::MAX && constraint[a] != first {
                println!("0");
                return;
            }
            constraint[a] = first;
            let mut i = 0;
            while i < g[a].len() && first == g[a][i].1 {
                del_set.insert((a, g[a][i].0));
                max_num[g[a][i].0] = max_num[g[a][i].0].min(first);
                i += 1;
            }
            i
        } else {
            1
        };
        for &(b, c) in &g[a][i..] {
            if constraint[b] != usize::MAX && constraint[b] != c {
                println!("0");
                return;
            }
            constraint[b] = c;
            del_set.insert((a, b));
        }
    }
    // eprintln!("{:?}", constraint);
    // eprintln!("{:?}", max_num);
    let mut num_set = vec![true; n];
    for &ci in &constraint {
        if ci != usize::MAX {
            if !num_set[ci] {
                println!("0");
                return;
            }
            num_set[ci] = false;
        }
    }
    let mut ans = MInt::new(1);
    // let mut g2 = vec![(usize::MAX, usize::MAX); n];
    for &(a, b, c) in &abc {
        let a = a - 1;
        let b = b - 1;
        let c = c - 1;
        if del_set.contains(&(a, b)) || del_set.contains(&(b, a)) {
            continue;
        }
        // assert_eq!(g2[a - 1].0, usize::MAX);
        // g2[a - 1] = (b - 1, c - 1);
        // assert_eq!(g2[b - 1].0, usize::MAX);
        // g2[b - 1] = (a - 1, c - 1);
        assert_eq!(constraint[a], usize::MAX);
        assert_eq!(constraint[b], usize::MAX);
        ans *= MInt::new(2);
        constraint[a] = c;
        if !num_set[c] {
            println!("0");
            return;
        }
        num_set[c] = false;
        max_num[b] = max_num[b].min(c);
    }
    // eprintln!("{:?}", constraint);
    // eprintln!("{:?}", max_num);
    // eprintln!("{:?}", num_set);
    // eprintln!("{}", ans);
    let mut max_num = constraint.iter().zip(&max_num).filter(|(&ci, _)| ci == usize::MAX).map(|(_, &mi)| mi).collect::<Vec<_>>();
    max_num.sort_unstable();
    let mut num = std::iter::once(0).chain(num_set.iter().map(|&b| b as usize)).collect::<Vec<_>>();
    // eprintln!("{:?}", max_num);
    for i in 1..=n {
        num[i] += num[i - 1];
    }
    // eprintln!("{:?}", num);
    for (i, &mi) in max_num.iter().enumerate() {
        ans *= MInt::new(num[mi].saturating_sub(i) as _);
    }
    println!("{}", ans);
    // eprintln!("{:?}", g2);

}
