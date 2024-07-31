use std::collections::HashMap;

use proconio::input;

fn reverse_digit(mut n: u64) -> Option<u64> {
    let mut m = 0;
    while n > 0 {
        if n % 10 == 0 {
            return None;
        }
        m = m * 10 + n % 10;
        n /= 10;
    }
    Some(m)
}

fn solve(n: u64, buf: &mut Vec<u64>, buf_rev: &mut Vec<u64>, note: &mut HashMap<u64, Option<Vec<u64>>>, keys: &[(u64, Option<u64>)]) -> bool {
    if let Some(v) = note.get(&n) {
        return if let Some(v) = v {
            buf.extend_from_slice(v);
            true
        } else {
            false
        };
    }

    for &(k, k_rev) in keys {
        if k != 1 && n % k == 0 {
            let n = n / k;
            if let Some(k_rev) = k_rev {
                if n % k_rev == 0 {
                    buf.push(k);
                    buf_rev.push(k_rev);
                    if solve(n / k_rev, buf, buf_rev, note, keys) {
                        note.insert(n, Some(buf.clone()));
                        return true;
                    }
                    buf.pop();
                    buf_rev.pop();
                }
            }
        }
    }
    note.insert(n, None);
    false
}

fn main() {
    input! {
        n: u64,
    }

    let mut note = HashMap::new();
    let mut keys = Vec::new();
    let mut i = 1;
    while i * i < n {
        if n % i == 0 {
            let i_rev = reverse_digit(i);
            if Some(i) == i_rev {
                note.insert(i, Some(vec![i]));
            }
            keys.push((i, i_rev));
            let j = n / i;
            let j_rev = reverse_digit(j);
            if Some(j) == reverse_digit(j) {
                note.insert(j, Some(vec![j]));
            }
            keys.push((j, j_rev));
        }
        i += 1;
    }
    if i * i == n {
        let i_rev = reverse_digit(i);
        if Some(i) == reverse_digit(i) {
            note.insert(i, Some(vec![i]));
        }
        keys.push((i, i_rev));
    }
    let mut buf = Vec::new();
    let mut buf_rev = Vec::new();
    if solve(n, &mut buf, &mut buf_rev, &mut note, &keys) {
        let mut iter = buf.iter().copied().chain(buf_rev.iter().copied().rev());
        print!("{}", iter.next().unwrap());
        for v in iter {
            print!("*{v}");
        }
        println!();
    } else {
        println!("-1");
    }
}
