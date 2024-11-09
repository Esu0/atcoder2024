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
        a: [[u32; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    }
    let mut g = vec![vec![false; n]; n];
    for &(x, y) in &xy {
        g[x - 1][y - 1] = true;
        g[y - 1][x - 1] = true;
    }

    let mut ans = u32::MAX;
    let mut buf = (0..n).collect::<Vec<_>>();
    permutation(n, 0, &mut buf, &mut |p| {
        let mut sum = 0;
        let mut prev = p[0];
        for &i in &p[1..] {
            if g[prev][i] {
                return;
            }
            prev = i;
        }
        for (j, &i) in p.iter().enumerate() {
            sum += a[i][j];
        }
        ans = ans.min(sum);
    });
    println!("{}", ans as i32);
}
