use std::ops;

use proconio::input;
use util::ModInt;

const MODULO: u64 = 998244353;
type MInt = ModInt<998244353>;
// a.len() == 2^k
fn hadamard_transform<T: ops::Add<Output = T> + ops::Sub<Output = T> + Copy>(a: &mut [T]) {
    assert!(a.len().is_power_of_two());
    let n = a.len();
    let mut b = 1;
    while b < n {
        let mut i = 0;
        while i < n {
            let mut j = 0;
            while j < b {
                let k = i + j;
                let x = a[k];
                let y = a[k + b];
                a[k] = x + y;
                a[k + b] = x - y;
                j += 1;
            }
            i += b << 1;
        }
        b <<= 1;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u32,
        a: [usize; n],
    }

    let a_max = 1 << 20;
    let mut count = vec![0i64; a_max];
    for &ai in &a {
        count[ai] += 1;
    }
    // eprintln!("{:?}", count);
    hadamard_transform(&mut count);
    // eprintln!("{:?}", count);
    for ci in count.iter_mut() {
        *ci = (n as i64 + *ci) / 2;
    }

    let mut f = vec![vec![0u64; m]; n + 1];
    let mut g = vec![vec![0u64; m]; n + 1];
    f[0][0] = 1;
    g[0][0] = 1;
    for i in 1..=n {
        for j in 0..m {
            let k = if j == 0 { m - 1 } else { j - 1 };
            f[i][j] = (f[i - 1][j] + f[i - 1][k]) % MODULO;
            g[i][j] = (g[i - 1][j] + MODULO - g[i - 1][k]) % MODULO;
        }
    }
    let mut ans_h = count.iter().map(|&ci| {
        let mut x = MInt::new(0);
        for j in 0..m {
            x += MInt::new(f[ci as usize][j] as _) * MInt::new(g[n - ci as usize][(m - j) % m] as _)
        }
        x
    }).collect::<Vec<_>>();
    hadamard_transform(&mut ans_h);

    let inv = MInt::new(a_max as _).inv();
    // eprintln!("{:?}", ans_h);
    let mut ans = MInt::new(0);
    for (i, &ai) in ans_h.iter().enumerate() {
        let i = MInt::new(i as _);
        ans += ai * inv * i.pow(k);
    }
    println!("{}", ans.get());
}
