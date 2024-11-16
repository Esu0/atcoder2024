use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut is_prime = vec![true; 1001];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=1000 {
        if is_prime[i] {
            let mut j = 2 * i;
            while j <= 1000 {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let mut ans = 0;
    a.sort_unstable_by_key(|&x| Reverse(x));
    while a.last() == Some(&0) {
        a.pop();
        ans += a.len();
    }

    let primes = is_prime.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i).collect::<Vec<_>>();
    a.iter_mut().for_each(|ai| {
        for &p in primes.iter() {
            let d = p * p;
            while *ai % d == 0 {
                *ai /= d;
            }
        }
    });

    let mut count = vec![0; 200001];
    for &ai in a.iter() {
        count[ai] += 1;
    }
    count.iter().for_each(|&c| {
        if c > 0 {
            ans += c * (c - 1) / 2;
        }
    });
    println!("{}", ans);
}
