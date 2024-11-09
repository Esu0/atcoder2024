use std::collections::HashMap;

use proconio::{input, marker};
use util::ModInt;
type MInt = ModInt<998244353>;
fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        mut s: [marker::Bytes; h],
    }
    if w > h {
        let mut new_s = vec![vec![0; h]; w];
        for (i, row) in s.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                new_s[j][i] = c;
            }
        }
        s = new_s;
        (h, w) = (w, h);
    }
    let mut dpi = HashMap::from([(0u32, MInt::new(1))]);
    let mut dpi_next = HashMap::new();
    let nums = [1, 2, 3];
    for i in 0..h {
        let it = if s[i][0] == b'?' { &nums[..] } else { &nums[(s[i][0] - b'1') as usize..][..1] };
        for (&k, &v) in dpi.iter() {
            let u_cell = k & 3;
            for &u in it {
                if u_cell != u {
                    let new_key = (k & !3) | u;
                    dpi_next.entry(new_key).and_modify(|e| *e += v).or_insert(v);
                }
            }
        }
        dpi.clone_from(&dpi_next);
        dpi_next.clear();
        for j in 1..w {
            let it = if s[i][j] == b'?' { &nums[..] } else { &nums[(s[i][j] - b'1') as usize..][..1] };
            for (&k, &v) in dpi.iter() {
                let l_cell = (k >> ((j - 1) * 2)) & 3;
                let u_cell = (k >> (j * 2)) & 3;
                for &l in it {
                    if l_cell != l && u_cell != l {
                        let new_key = (k & !(3 << (j * 2))) | (l << (j * 2));
                        dpi_next.entry(new_key).and_modify(|e| *e += v).or_insert(v);
                    }
                }
            }
            // eprintln!("{:?}", dpi_next);
            dpi.clone_from(&dpi_next);
            dpi_next.clear();
        }
    }
    let ans = dpi.values().fold(MInt::new(0), |acc, &v| acc + v);
    println!("{}", ans);
}
