use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut set = (0..n).map(|i| (i, (i, 1usize))).collect::<BTreeMap<_, _>>();
    let mut count = vec![1; n];
    for _ in 0..q {
        input! { t: u8 }
        match t {
            1 => {
                input! { x: usize, c: usize }
                let x = x - 1;
                let c = c - 1;
                let (&k, &(col, cou)) = set.range(..=x).last().unwrap();
                count[col] -= cou;
                count[c] += cou;
                set.remove(&k);
                let mut k = k;
                let col = c;
                let mut count = cou;
                if let Some(prev) = set.range(..k).last() {
                    if prev.1 .0 == c {
                        count += prev.1 .1;
                        k = *prev.0;
                        set.remove(&k);
                    }
                }
                if let Some(nxt) = set.range(k + 1..).next() {
                    if nxt.1 .0 == c {
                        count += nxt.1 .1;
                        let key = *nxt.0;
                        set.remove(&key);
                    }
                }
                set.insert(k, (col, count));
            }
            2 => {
                input! { c: usize }
                println!("{}", count[c - 1]);
            }
            _ => unreachable!(),
        }
    }
}
