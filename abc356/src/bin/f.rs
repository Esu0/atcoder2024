use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        q: usize,
        k: u64,
    }
    let mut seq = BTreeSet::new();
    let mut ranges = BTreeMap::<u64, (u64, usize)>::new();

    for _ in 0..q {
        input! {
            t: u8,
            x: u64,
        }
        if t == 1 {
            let inserted = seq.insert(x);
            if inserted {
                ranges.insert(x, (x, 1));
                if let Some((_, (r, c))) = ranges.range_mut(..x).last() {
                    if x > *r {
                        *c += 1;
                        ranges.remove(&x);
                        continue;
                    } else if x - *r <= k {
                        *r = x;
                        *c += 1;
                        ranges.remove(&x);
                    }
                }
                if let Some((&l, &(r, c))) = ranges.range(x + 1..).next() {
                    if l - x <= k {
                        let (_, prev) = ranges.range_mut(..=x).last().unwrap();
                        prev.0 = r;
                        prev.1 += c;
                        ranges.remove(&l);
                    }
                }
            } else {
                seq.remove(&x);
                let (&l, (r, c)) = ranges.range_mut(..=x).last().unwrap();
                let prev = seq.range(..x).last().copied();
                let next = seq.range(x + 1..).next().copied();
                if *r == x {
                    *c -= 1;
                    if *c == 0 {
                        ranges.remove(&l);
                    } else {
                        *r = prev.unwrap();
                    }
                } else if l == x {
                    if *c == 1 {
                        ranges.remove(&l);
                    } else {
                        let r = *r;
                        let c = *c - 1;
                        ranges.remove(&l);
                        ranges.insert(next.unwrap(), (r, c));
                    }
                } else {
                    let prev = prev.unwrap();
                    let next = next.unwrap();
                }
            }
        }
        if t == 2 {

        }
    }
}
