use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        tp: [(u32, usize); m],
        q: usize,
    }

    let mut record = vec![vec![]; n];
    for &(t, p) in &tp {
        record[p - 1].push(t);
    }
    let record = record
        .into_iter()
        .map(|v| v.chunks(2).map(|w| [w[0], w[1]]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut answers = HashMap::<(usize, usize), u32>::new();

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
        }
        if let Some(&a) = answers.get(&(a, b)) {
            println!("{}", a);
            continue;
        }
        
    }
}
