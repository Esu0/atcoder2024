use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [marker::Bytes; h],
        t: [marker::Bytes; h],
    }
    let mut s_transposed = vec![vec![0; h]; w];
    for (i, row) in s.iter().enumerate() {
        for (j, row_t) in s_transposed.iter_mut().enumerate() {
            row_t[i] = row[j];
        }
    }
    let mut t_transposed = vec![vec![0; h]; w];
    for (i, row) in t.iter().enumerate() {
        for (j, row_t) in t_transposed.iter_mut().enumerate() {
            row_t[i] = row[j];
        }
    }
    s_transposed.sort_unstable();
    t_transposed.sort_unstable();
    if s_transposed == t_transposed {
        println!("Yes");
    } else {
        println!("No");
    }
}
