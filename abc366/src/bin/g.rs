use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut adj_mat_buf = [0; 60];
    let adj_mat = &mut adj_mat_buf[..n];
    for &(u, v) in &uv {
        adj_mat[u - 1] |= 1u64 << (v - 1);
        adj_mat[v - 1] |= 1 << (u - 1);
    }

    // let non_zero_v = adj_mat
    //     .iter()
    //     .enumerate()
    //     .filter(|&(_, &x)| x != 0)
    //     .map(|(i, _)| i)
    //     .collect::<Vec<_>>();

    // let n = non_zero_v.len();
    // if n == 0 {
    //     println!("Yes");
    //     for _ in 0..n_orig {
    //         print!("1 ");
    //     }
    //     println!();
    //     return;
    // }

    // let mut non_zero_pos = vec![0; n_orig];
    // for (i, &x) in non_zero_v.iter().enumerate() {
    //     non_zero_pos[x] = i;
    // }
    // let adj_mat = &mut adj_mat[..n];
    // adj_mat.fill(0);
    // for &(u, v) in &uv {
    //     let u = non_zero_pos[u - 1];
    //     let v = non_zero_pos[v - 1];
    //     adj_mat[u] |= 1 << v;
    //     adj_mat[v] |= 1 << u;
    // }

    // adj_mat.iter().for_each(|&x| eprintln!("{:01$b}", x, n));
    // eprintln!();
    let mut b = 1u64;
    let mut k = 0;
    for _ in 0..n {
        if let Some(j) = adj_mat[k..].iter().position(|&x| x & b != 0) {
            let j = k + j;
            adj_mat.swap(k, j);
            let rowk = adj_mat[k];
            adj_mat[k + 1..].iter_mut().for_each(|x| {
                if *x & b != 0 {
                    *x ^= rowk
                }
            });
            k += 1;
        }
        b <<= 1;
        // adj_mat.iter().for_each(|&x| eprintln!("{:01$b}", x, n));
        // eprintln!();
    }

    // adj_mat.iter().for_each(|&x| eprintln!("{:01$b}", x, n));
    // eprintln!();

    let zeros = adj_mat.iter().filter(|&&x| x == 0).count();
    let mut basis = vec![0; zeros];

    let mut b = 1u64 << (n - 1);
    let mut k = n;
    let mut i = 0;
    for &x in adj_mat.iter().rev() {
        if x == 0 {
            continue;
        }
        while k > x.trailing_zeros() as usize + 1 {
            basis[i] |= b;
            b >>= 1;
            k -= 1;
            i += 1;
        }
        k -= 1;
        for base in &mut basis {
            *base |= ((x & *base).count_ones() as u64 & 1) << k;
        }
        b >>= 1;
    }
    while k != 0 {
        basis[i] |= b;
        b >>= 1;
        k -= 1;
        i += 1;
    }
    // basis.iter().for_each(|&x| eprintln!("{:01$b}", x, n));
    // eprintln!();

    if basis.iter().fold(0, |acc, &x| acc | x) == (1u64 << n) - 1 {
        println!("Yes");
        // let mut b = 1u64;
        for k in 0..n {
            let mut ans = 0;
            for (i, &x) in basis.iter().enumerate() {
                let b = (x >> k) & 1;
                ans |= b << i;
            }
            print!("{} ", ans);
        }
        println!();
    } else {
        println!("No");
    }
}
