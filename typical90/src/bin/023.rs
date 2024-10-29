use std::collections::HashMap;

use proconio::{input, marker};
use util::ModInt;
type MInt = ModInt<1000000007>;

macro_rules! chsum {
    ($d:expr, $s:expr) => {
        {
            let tmp = $d + $s;
            $d = tmp;
        }
    };
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [marker::Bytes; h],
    }

    let c = c.iter().map(|ci| {
        let mut b = 0u32;
        for (j, &cij) in ci.iter().enumerate() {
            if cij == b'#' {
                b |= 1 << j;
            }
        }
        b
    }).collect::<Vec<_>>();
    // let index_set = {
    //     let mut v1 = vec![0usize];
    //     let mut v2 = vec![1];
    //     for i in 1..w {
    //         std::mem::swap(&mut v1, &mut v2);
    //         v1.extend_from_slice(&v2);
    //         for v2j in &mut v2 {
    //             *v2j |= 1 << i;
    //         }
    //     }
    //     v1.extend_from_slice(&v2);
    //     v1.sort_unstable();
    //     v1
    // };
    

    let mut dpi = HashMap::from([(0u32, MInt::new(1))]);
    let mut dpi_next = HashMap::new();
    let mask = (1 << w) - 1;
    for &ci in c.iter() {
        // dpi_next.clone_from(&dpi);
        dpi_next.clear();
        for (&j, &dpij) in dpi.iter() {
            let j = j & mask;
            if ci & 1 == 0 && j & 0b11 == 0 {
                dpi_next.entry((j << 1) | 1).and_modify(|dpij_next| chsum!(*dpij_next, dpij)).or_insert(dpij);
            }
            dpi_next.entry(j << 1).and_modify(|dpij_next| chsum!(*dpij_next, dpij)).or_insert(dpij);
        }
        std::mem::swap(&mut dpi, &mut dpi_next);
        for j in 1..w {
            dpi_next.clear();
            for (&k, &dpik) in dpi.iter() {
                if ci & (1 << j) == 0 && k & (0b1111 << (j - 1)) == 0 {
                    dpi_next.entry(k | (1 << j)).and_modify(|dpik_next| chsum!(*dpik_next, dpik)).or_insert(dpik);
                }
                dpi_next.entry(k & !(1 << j)).and_modify(|dpik_next| chsum!(*dpik_next, dpik)).or_insert(dpik);
            }
            std::mem::swap(&mut dpi, &mut dpi_next);
        }
    }
    // eprintln!("{:?}", dpi);
    let ans = dpi.values().fold(MInt::new(0), |acc, &dpij| acc + dpij);
    println!("{}", ans);
}
