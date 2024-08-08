use std::sync::atomic::{AtomicUsize, Ordering::*};

use proconio::input_interactive;

static Q: AtomicUsize = AtomicUsize::new(0);

fn query(a: u8, b: u8) -> bool {
    if Q.load(Relaxed) == 0 {
        panic!("info: query limit exceeded");
    } else {
        Q.fetch_sub(1, Relaxed);
    }
    println!("? {} {}", (b'A' + a) as char, (b'A' + b) as char);
    input_interactive! {
        c: char,
    }
    match c {
        '>' => true,
        '<' => false,
        _ => unreachable!(),
    }
}

fn solve26(v: &mut [u8], buf: &mut [u8]) {
    let n = v.len();
    if n == 1 {
        return;
    }
    if n == 2 {
        if query(v[0], v[1]) {
            v.swap(0, 1);
        }
        return;
    }
    let m1 = n / 2;
    let m2 = n - m1;
    let (l, r) = v.split_at_mut(m1);
    let (buf_l, buf_r) = buf.split_at_mut(m1);
    solve26(l, buf_l);
    solve26(r, buf_r);

    buf_l.copy_from_slice(l);
    buf_r.copy_from_slice(r);
    let mut i = 0;
    let mut j = 0;
    for k in 0..n {
        if i < m1 && (j >= m2 || query(buf_r[j], buf_l[i])) {
            v[k] = buf_l[i];
            i += 1;
        } else {
            v[k] = buf_r[j];
            j += 1;
        }
    }
}

fn permutation(n: usize, buf: &mut [u8], f: &mut impl FnMut(&[u8])) {
    if n == 1 {
        f(buf);
        return;
    }
    for i in 0..n {
        buf.swap(i, n - 1);
        permutation(n - 1, buf, f);
        buf.swap(i, n - 1);
    }
}

fn solve5(v: &mut [u8]) {
    let mut permutation_set1 = vec![];
    let mut permutation_set2 = vec![];
    let mut permutation_set3 = vec![];
    let mut buf = [0, 1, 2, 3, 4];
    permutation(5, &mut buf, &mut |x| permutation_set1.push(std::array::from_fn::<_, 5, _>(|i| x[i])));
    while permutation_set1.len() != 1 {
        let mut min_diff = usize::MAX;
        let mut compare = (0, 0);
        let mut next_set = (vec![], vec![]);
        for i in 0..5 {
            for j in i + 1..5 {
                permutation_set2.clear();
                permutation_set3.clear();
                for &p in &permutation_set1 {
                    if p[0] == 5 {
                        continue;
                    }
                    if p[i] > p[j] {
                        permutation_set2.push(p);
                    } else {
                        permutation_set3.push(p);
                    }
                }
                let diff = permutation_set2.len().abs_diff(permutation_set3.len());
                if diff < min_diff {
                    min_diff = diff;
                    next_set = (permutation_set2.clone(), permutation_set3.clone());
                    compare = (i, j);
                }
            }
        }
        let (i, j) = compare;
        permutation_set1 = if query(i as u8, j as u8) {
            next_set.0.clone()
        } else {
            next_set.1.clone()
        };
    }
    // eprintln!("{:?}", permutation_set1[0]);
    for (i, &p) in permutation_set1[0].iter().enumerate() {
        v[p as usize] = i as u8;
    }
}
fn main() {
    input_interactive! {
        n: usize,
        q: usize,
    }
    Q.store(q, Relaxed);
    let mut v = (0..n as u8).collect::<Vec<_>>();
    let mut buf = vec![0; n];
    if n == 5 {
        solve5(&mut v);
    } else if n == 26 {
        solve26(&mut v, &mut buf);
    }
    v.iter_mut().for_each(|x| *x += b'A');
    println!("! {}", std::str::from_utf8(&v).unwrap());
}
