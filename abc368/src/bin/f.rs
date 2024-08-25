use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    let a_max = 200000;
    let mut prime = vec![true; a_max + 1];
    prime[0] = false;
    prime[1] = false;
    for i in 2..2000 {
        if prime[i] {
            let mut j = i * 2;
            while j <= a_max {
                prime[j] = false;
                j += i;
            }
        }
    }
    let prime_set = prime[..1000].iter().enumerate().filter(|&(_, &p)| p).map(|(i, _)| i as u64).collect::<Vec<_>>();
    for ai_d in &mut a {
        let mut ai = *ai_d;
        let mut c = 0;
        for &p in &prime_set {
            while ai % p == 0 {
                ai /= p;
                c += 1;
            }
            if ai == 1 {
                break;
            }
        }
        if ai != 1 {
            c += 1;
        }
        *ai_d = c;
    }
    if a.iter().fold(0, |acc, &x| acc ^ x) == 0 {
        println!("Bruno");
    } else {
        println!("Anna");
    }
}
