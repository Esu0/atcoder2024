use std::collections::HashSet;

use proconio::{input, marker};
use union_find::UnionFind;

fn main() {
    input! {
        _n: usize,
        s1: marker::Bytes,
        s2: marker::Bytes,
    }

    let mut uf = UnionFind::new(vec![(); 26]);
    let mut digit = [10u64; 26];
    let mut alphabets = HashSet::new();
    for (&c1, &c2) in s1.iter().zip(&s2) {
        let c1d = c1.is_ascii_digit();
        let c2d = c2.is_ascii_digit();
        if c1d && c2d {
            continue;
        }
        if c1d {
            let ind = uf.find_rc((c2 - b'A') as usize);
            alphabets.insert((c2 - b'A') as usize);
            digit[ind] = 1;
        } else if c2d {
            let ind = uf.find_rc((c1 - b'A') as usize);
            alphabets.insert((c1 - b'A') as usize);
            digit[ind] = 1;
        } else {
            let c1i = (c1 - b'A') as usize;
            let c2i = (c2 - b'A') as usize;
            alphabets.insert(c1i);
            alphabets.insert(c2i);
            let old1 = uf.find_rc(c1i);
            let old2 = uf.find_rc(c2i);
            if uf.unite(c1i, c2i) {
                let new = digit[old1].min(digit[old2]);
                digit[uf.find_rc(c1i)] = new;
            }
        }
    }

    if s1[0].is_ascii_alphabetic() {
        let i = uf.find_rc((s1[0] - b'A') as usize);
        digit[i] = digit[i].min(9);
    }

    if s2[0].is_ascii_alphabetic() {
        let i = uf.find_rc((s2[0] - b'A') as usize);
        digit[i] = digit[i].min(9);
    }

    let mut ind_set = HashSet::new();
    for &c in alphabets.iter() {
        ind_set.insert(uf.find_rc(c));
    }
    let mut ans = 1;
    for &ind in ind_set.iter() {
        ans *= digit[ind];
    }
    println!("{}", ans);
}
