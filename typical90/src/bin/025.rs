use proconio::input;

fn digit_prod(n: u64) -> u64 {
    let mut n = n;
    let mut result = 1;
    while n > 0 {
        result *= n % 10;
        n /= 10;
    }
    result
}

fn main() {
    input! {
        n: u64,
        b: u64,
    }

    let mut count = 0;
    if b > n {
        println!("0");
        return;
    }
    if digit_prod(b) == 0 {
        count += 1;
    }
    for e2 in 0..=33 {
        let p = 2u64.pow(e2);
        for e3 in 0..=22 {
            let Some(p) = p.checked_mul(3u64.pow(e3)) else {
                break;
            };
            for e5 in 0..=11 {
                let Some(p) = p.checked_mul(5u64.pow(e5)) else {
                    break;
                };
                for e7 in 0..=11 {
                    let Some(p) = p.checked_mul(7u64.pow(e7)) else {
                        break;
                    };

                    let Some(m) = p.checked_add(b) else {
                        break;
                    };
                    if m <= n && digit_prod(m) == p {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}
