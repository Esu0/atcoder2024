use proconio::input;

fn permutation(n: usize, i: usize, buf: &mut [usize], f: &mut impl FnMut(&[usize])) {
    if i == n {
        f(buf);
        return;
    }

    for j in i..n {
        buf.swap(i, j);
        permutation(n, i + 1, buf, f);
        buf.swap(i, j);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvt: [(usize, usize, u64); m],
        q: u16,
    }

    let mut adj_mat = vec![vec![u64::MAX; n]; n];
    for &(u, v, t) in &uvt {
        adj_mat[u - 1][v - 1] = adj_mat[u - 1][v - 1].min(t);
        adj_mat[v - 1][u - 1] = adj_mat[u - 1][v - 1].min(t);
    }

    for (i, rowi) in adj_mat.iter_mut().enumerate() {
        rowi[i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                adj_mat[i][j] = adj_mat[i][j].min(adj_mat[i][k].saturating_add(adj_mat[k][j]));
            }
        }
    }

    for _ in 0..q {
        input! {
            k: usize,
            b: [usize; k],
        }
        let e = 1u32 << k;
        let mut buf = [0; 5];
        let mut min = u64::MAX;
        for d in 0..e {
            let buf = &mut buf[..k];
            buf.clone_from_slice(&b);
            permutation(k, 0, buf, &mut |buf| {
                let mut sum = 0;
                let mut prev = 0;
                let mut mask = 1;
                for i in 0..k {
                    let (u, v, t) = uvt[buf[i] - 1];
                    if d & mask != 0 {
                        // u -> vの向き
                        sum += adj_mat[prev][u - 1] + t;
                        prev = v - 1;
                    } else {
                        // v -> uの向き
                        sum += adj_mat[prev][v - 1] + t;
                        prev = u - 1;
                    }
                    mask <<= 1;
                }
                sum += adj_mat[prev][n - 1];
                min = min.min(sum);
            });
        }
        println!("{}", min);
    }
}
