use proconio::input;
fn permutations(buf: &mut [usize], n: usize, i: usize, f: &mut impl FnMut(&[usize])) {
    if i == n {
        f(buf);
        return;
    }
    for j in i..n {
        buf.swap(i, j);
        permutations(buf, n, i + 1, f);
        buf.swap(i, j);
    }
}

fn main() {
    input! {
        n: usize,
        mg: usize,
        uv: [(usize, usize); mg],
        mh: usize,
        ab: [(usize, usize); mh],
    }
    let mut a = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        for j in i + 1..n {
            input! {
                aij: u64,
            }
            a[i][j] = aij;
            a[j][i] = aij;
        }
    }

    // eprintln!("{:?}", a);
    let mut arr = std::array::from_fn::<_, 8, _>(|i| i);
    let buf = &mut arr[..n];
    let mut maps = vec![];
    permutations(buf, n, 0, &mut |p| {
        maps.push(p.to_vec());
    });

    let mut g = vec![vec![false; n]; n];
    for (u, v) in uv {
        g[u - 1][v - 1] = true;
        g[v - 1][u - 1] = true;
    }
    let mut h = vec![vec![false; n]; n];
    for (a, b) in ab {
        h[a - 1][b - 1] = true;
        h[b - 1][a - 1] = true;
    }
    // eprintln!("{:?}", &maps[0..10]);
    let mut ans = u64::MAX;
    for map in &maps {
        let mut cost = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                if g[map[i]][map[j]] != h[i][j] {
                    cost += a[i][j];
                }
            }
        }
        ans = ans.min(cost);
    }
    println!("{}", ans);
}
