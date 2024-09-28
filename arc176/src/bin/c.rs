use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }
    let mut cond = vec![vec![]; n];
    for &(a, b, c) in &abc {
        cond[c - 1].push((a - 1, b - 1));
    }
    let mut w = 0;
    let mut flg = vec![false; n];
    for v in &cond {
        if v.is_empty() {
            w += 1;
            continue;
        }
        if v.len() == 1 {
            let (a, b) = v[0];
            
        }
    }
}
