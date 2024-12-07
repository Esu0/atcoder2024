use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut max_m = 1;
    while max_m * max_m <= n {
        max_m += 1;
    }
    max_m -= 1;
    let mut is_prime = vec![true; 2000001];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=2000000 {
        if is_prime[i] {
            let mut j = i * 2;
            while j <= 2000000 {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let primes = is_prime
        .iter()
        .enumerate()
        .filter(|&(i, &flg)| flg)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut ans = 0;
    for (i, &pi) in primes.iter().enumerate() {
        let j = primes[i + 1..].partition_point(|&x| x * pi <= max_m);
        ans += j;
    }
    for &p in &primes {
        let p2 = p * p;
        let Some(p4) = p2.checked_mul(p2) else {
            break;
        };
        if p4 > max_m {
            break;
        }
        ans += 1;
    }
    println!("{ans}");
}
