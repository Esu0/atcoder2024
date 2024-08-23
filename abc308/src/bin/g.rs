use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut set = BTreeMap::<_, u32>::new();
    let mut ans_set = BTreeMap::<_, u32>::new();
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    x: u64,
                }
                let l = set.range(..x).last().map(|(&l, _)| l);
                let r = set.range(x..).next().map(|(&r, _)| r);
                if let Some(l) = l {
                    ans_set.entry(l ^ x).and_modify(|e| *e += 1).or_insert(1);
                }
                if let Some(r) = r {
                    ans_set.entry(r ^ x).and_modify(|e| *e += 1).or_insert(1);
                }
                if let (Some(l), Some(r)) = (l, r) {
                    let c = ans_set.get_mut(&(l ^ r)).unwrap();
                    if *c == 1 {
                        ans_set.remove(&(l ^ r));
                    } else {
                        *c -= 1;
                    }
                }
                set.entry(x).and_modify(|e| *e += 1).or_insert(1);
            }
            2 => {
                input! {
                    x: u64,
                }
                let c = set.get_mut(&x).unwrap();
                if *c == 1 {
                    set.remove(&x);
                } else {
                    *c -= 1;
                }
                let l = set.range(..x).last().map(|(&l, _)| l);
                let r = set.range(x..).next().map(|(&r, _)| r);
                if let Some(l) = l {
                    let c = ans_set.get_mut(&(l ^ x)).unwrap();
                    if *c == 1 {
                        ans_set.remove(&(l ^ x));
                    } else {
                        *c -= 1;
                    }
                }
                if let Some(r) = r {
                    let c = ans_set.get_mut(&(r ^ x)).unwrap();
                    if *c == 1 {
                        ans_set.remove(&(r ^ x));
                    } else {
                        *c -= 1;
                    }
                }
                if let (Some(l), Some(r)) = (l, r) {
                    ans_set.entry(l ^ r).and_modify(|e| *e += 1).or_insert(1);
                }
            }
            3 => {
                let ans = *ans_set.first_key_value().unwrap().0;
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
