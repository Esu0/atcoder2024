use std::collections::HashMap;

use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut map = HashMap::from([(0i64, MInt::new(1))]);
    let mut s = 0;
    // let mut ans = 0;
    let mut sum = MInt::new(1);
    for &ai in &a[..n - 1] {
        // eprintln!("{:?}", map);
        // dbg!(sum);
        let next_s = s + ai;
        // dbg!(next_s - k);
        let a = sum - if let Some(t) = map.get(&(next_s - k)) {
            *t
        } else {
            MInt::new(0)
        };
        map.entry(next_s).and_modify(|t| *t += a).or_insert(a);
        sum += a;
        s = next_s;
    }
    println!("{}", sum - map.get(&(s + *a.last().unwrap() - k)).copied().unwrap_or_default());
}
