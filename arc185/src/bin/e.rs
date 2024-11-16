use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut div = vec![vec![]; 100_001];
    for i in 1..=100_000 {
        let mut j = i;
        while j <= 100_000 {
            div[j].push(i);
            j += i;
        }
    }
    let mut phi = (0..=100_000).map(MInt::new).collect::<Vec<_>>();
    for i in 2..=100_000 {
        if phi[i].get() == i as u32 {
            let mut j = 100_000 / i;
            while j > 0 {
                let tmp = phi[j];
                phi[j * i] -= tmp;
                j -= 1;
            }
        }
    }

    // eprintln!("{:?}", &phi[..30]);
    let mut b = vec![MInt::new(0); 100_001];
    let mut pow2 = MInt::new(1);
    let mut prev = MInt::new(0);
    for &ai in &a {
        let mut ans = prev + prev;
        for &d in &div[ai] {
            ans += phi[d] * b[d];
        }
        println!("{}", ans);
        prev = ans;
        for &d in &div[ai] {
            b[d] += pow2;
        }
        pow2 += pow2;
    }
}
