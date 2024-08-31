use core::panic;
use std::{sync::atomic::{AtomicUsize, Ordering::*}, vec};

use proconio::input_interactive;

static COUNT: AtomicUsize = AtomicUsize::new(25000);

fn decrement_count() {
    let c = COUNT.load(Relaxed);
    if c == 0 {
        panic!("query limit exceeded");
    }
    COUNT.store(c - 1, Relaxed);
}

fn add(i: usize, j: usize) -> usize {
    decrement_count();
    println!("+ {} {}", i + 1, j + 1);
    input_interactive! { p: isize }
    if p == -1 {
        panic!("wrong answer");
    }
    p as usize - 1
}

// true if X_i < X_j
fn compare(i: usize, j: usize) -> bool {
    decrement_count();
    println!("? {} {}", i + 1, j + 1);
    input_interactive! { q: i8 }
    if q == -1 {
        panic!("wrong answer");
    }
    q == 1
}

fn merge_sort(a: &mut [usize]) {
    let n = a.len();
    let mut buf = vec![0; n];
    merge_sort_rec(a, &mut buf);
}

fn merge_sort_rec(a: &mut [usize], buf: &mut [usize]) {
    let n = a.len();
    if n == 1 {
        return;
    }
    if n == 2 {
        if compare(a[1], a[0]) {
            a.swap(0, 1);
        }
        return;
    }
    let m = n / 2;
    let (a1, a2) = a.split_at_mut(m);
    let (buf1, buf2) = buf.split_at_mut(m);
    merge_sort_rec(a1, buf1);
    merge_sort_rec(a2, buf2);

    let n1 = m;
    let n2 = n - m;
    let mut i = 0;
    let mut j = 0;
    for bufk in &mut *buf {
        if i < n1 && (j == n2 || compare(a1[i], a2[j])) {
            *bufk = a1[i];
            i += 1;
        } else {
            *bufk = a2[j];
            j += 1;
        }
    }
    a.copy_from_slice(buf);
}

fn main() {
    input_interactive! {
        n: usize,
    }
    let mut a = (0..n).collect::<Vec<_>>();
    merge_sort(&mut a);
    // eprintln!("{:?}", a);
    for i in (1..n).rev() {
        let ai = a.pop().unwrap();
        let new = add(a[0], ai);
        a.remove(0);
        // a: 長さi - 1のソート済み配列
        if i == 1 {
            break;
        }
        let mut l = 0;
        let mut r = i - 1;
        while r - l > 1 {
            let m = (l + r) / 2;
            if compare(a[m], new) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if r > l {
            if compare(a[l], new) {
                a.insert(l + 1, new);
            } else {
                a.insert(l, new);
            }
        } else {
            a.insert(l, new);
        }
    }
    println!("!");
}
