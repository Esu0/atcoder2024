use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }
    let mut ans = vec![vec![usize::MAX; n]; n];
    // let mut p_inv = vec![usize::MAX; n];
    // let mut q_inv = vec![usize::MAX; n];
    // for (i, &pi) in p.iter().enumerate() {
    //     p_inv[pi - 1] = i;
    // }
    // for (i, &qi) in q.iter().enumerate() {
    //     q_inv[qi - 1] = i;
    // }
    let mut i = 0;
    let mut j = n;
    while j > i {
        j -= 1;
        let row = p[j] - 1;
        let col = q[j] - 1;
        for k in 0..n {
            if ans[row][k] == usize::MAX {
                ans[row][k] = 1;
            }
            if ans[k][col] == usize::MAX {
                ans[k][col] = 1;
            }
        }
        if j <= i {
            break;
        }
        let row = p[i] - 1;
        let col = q[i] - 1;
        for k in 0..n {
            if ans[row][k] == usize::MAX {
                ans[row][k] = 0;
            }
            if ans[k][col] == usize::MAX {
                ans[k][col] = 0;
            }
        }
        i += 1;
    }
    for row in &ans {
        for &c in row {
            print!("{}", c);
        }
        println!();
    }
}
