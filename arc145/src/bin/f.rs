use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;

// m % p == 0 のとき
// m <- m / p
fn solve1(
    m: usize,
    n: usize,
    p: usize,
    divisors: &[usize],
    fact: &[MInt],
    fact_inv: &[MInt],
    primes: &[bool],
) -> Vec<MInt> {
    let mut g = vec![MInt::new(0); p + 1];
    for &d in divisors {
        let a = p / d;
        if n % a == 0 {
            let tmp = fact[d * m] * fact_inv[n * d / p] * fact_inv[m * d - n * d / p];
            g[d] = if (d * n / p + n) % 2 == 1 {
                MInt::new(0) - tmp
            } else {
                tmp
            };
        }
    }
    let mut w_inv_sum = vec![MInt::new(0); p + 1];
    let p_inv = MInt::new(p as _).inv();
    (0..p)
        .map(|j| {
            for &k in divisors {
                w_inv_sum[k] = if j % (p / k) == 0 {
                    MInt::new((p / k) as _)
                } else {
                    MInt::new(0)
                };
            }
            // eprintln!("{:?}", w_inv_sum);
            // 約数メビウス変換
            for &k in &divisors[1..] {
                if primes[k] {
                    for i in 1..=p / k {
                        let tmp = w_inv_sum[i] - w_inv_sum[k * i];
                        w_inv_sum[i] = tmp;
                    }
                }
            }
            // eprintln!("{:?}", w_inv_sum);
            divisors
                .iter()
                .map(|&d| g[d] * w_inv_sum[d])
                .fold(MInt::new(0), |acc, x| acc + x)
                * p_inv
        })
        .collect::<Vec<_>>()
}
fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
    }
    let m = n + m;
    let mut gcd_count = vec![0; p + 1];
    for (k, item_k) in gcd_count.iter_mut().enumerate().skip(1) {
        if p % k == 0 {
            *item_k = p / k; // kを割り切るiの数 = kの倍数の数
        }
    }
    // eprintln!("{:?}", gcd_count);
    let divisors = (1..=p).filter(|&k| gcd_count[k] > 0).collect::<Vec<_>>();
    let mut primes = vec![true; p + 1];
    // 約数メビウス変換
    for &d in &divisors[1..] {
        if !primes[d] {
            continue;
        }
        gcd_count[1] -= gcd_count[d];
        for i in 2..=p / d {
            primes[d * i] = false;
            gcd_count[i] -= gcd_count[d * i];
        }
    }
    // eprintln!("{:?}", gcd_count);

    let mut factrials = vec![MInt::new(1); m + 1];
    for i in 1..=m {
        factrials[i] = MInt::new(i as _) * factrials[i - 1];
    }
    let factrials_inv = factrials.iter().map(|&x| x.inv()).collect::<Vec<_>>();
    // let a = solve1(m / p, n, p, &divisors, &factrials, &factrials_inv, &primes);
    // eprintln!("{:?}", a);
    let l = m % p;
    let mut mi = m - l;
    let mut dp = vec![vec![vec![MInt::new(0); p]; l + 1]; l + 1];
    dp[0][0][0] = MInt::new(1);
    for i in 0..l {
        for j in 0..=i {
            for k in 0..p {
                let tmp = dp[i][j][k];
                dp[i + 1][j][k] += tmp;
                dp[i + 1][j + 1][(k + mi) % p] += tmp;
            }
        }
        mi += 1;
    }
    // eprintln!("{:?}", dp);
    let ans2 = dp.into_iter().last().unwrap();
    let offset = n * (n - 1) / 2 % p;
    if m < p {
        for &a in ans2[n][offset..].iter().chain(&ans2[n][..offset]) {
            print!("{} ", a);
        }
        return;
    }
    let ans1 = (n.saturating_sub(l)..=n)
        .map(|i| (i, solve1(m / p, i, p, &divisors, &factrials, &factrials_inv, &primes)));
    let mut ans = vec![MInt::new(0); p];
    for (i, ans1i) in ans1 {
        // eprintln!("{:?}", ans1i);
        for (j, &ans1ij) in ans1i.iter().enumerate() {
            for (k, &ans2k) in ans2[n - i].iter().enumerate() {
                ans[(j + k) % p] += ans1ij * ans2k;
            }
        }
    }
    for &a in ans[offset..].iter().chain(&ans[..offset]) {
        print!("{} ", a);
    }
}
