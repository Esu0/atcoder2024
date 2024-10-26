use std::vec;

use proconio::input;

fn mpow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    input! {
        n: usize,
        k: u64,
        mut p: [usize; n],
    }
    p.iter_mut().for_each(|x| *x -= 1);
    let mut i = 0;
    let mut stack = vec![];
    let mut buf = vec![];
    let mut flg = vec![false; n];
    while i < n {
        if flg[i] {
            i += 1;
            continue;
        }
        stack.clear();
        let mut j = i;
        stack.push(i);
        while p[j] != i {
            j = p[j];
            flg[j] = true;
            stack.push(j);
        }
        // eprintln!("{:?}", stack);
        buf.clone_from(&stack);
        let rot_count = mpow(2, k, stack.len() as u64);
        for i in 0..stack.len() {
            p[buf[i]] = buf[(i + rot_count as usize) % stack.len()];
        }
        i += 1;
    }
    // eprintln!("{:?}", t);
    for &pi in &p {
        print!("{} ", pi + 1);
    }
    println!();
}
